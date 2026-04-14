use crate::{
    config::SessionConfig,
    domain::{EventEnvelope, GenerationProvenance, GeneratorFamily},
};
use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use clap::ValueEnum;
use rand::RngCore;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    #[value(name = "local-demo")]
    LocalDemo,
    #[value(name = "openai-compatible")]
    OpenAiCompatibleApi,
    #[value(name = "anthropic")]
    AnthropicApi,
    #[value(name = "openai-consumer")]
    OpenAiConsumer,
    #[value(name = "claude-consumer")]
    ClaudeConsumer,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterType {
    LocalDemo,
    Api,
    Consumer,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentalGenerationMode {
    #[value(name = "prompt-versioned")]
    PromptVersioned,
    #[value(name = "personalized")]
    Personalized,
    #[value(name = "consumer-session")]
    ConsumerSession,
}

#[derive(Clone, Debug)]
pub struct ExperimentalConfig {
    pub provider: ProviderKind,
    pub mode: ExperimentalGenerationMode,
    pub provider_profile: Option<String>,
    pub prompt_asset: Option<String>,
    pub prompt_version: Option<String>,
    pub personalization_profile: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
    pub session_file: Option<String>,
    pub bootstrap_command: Option<String>,
    pub store_path: Option<String>,
    pub disable_cache: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderProfile {
    pub id: String,
    pub provider_kind: ProviderKind,
    pub base_url: Option<String>,
    pub model: String,
    pub adapter_type: AdapterType,
    pub prompt_caching: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAsset {
    pub id: String,
    pub owner: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAssetVersion {
    pub prompt_asset_id: String,
    pub version: String,
    pub tool_schema_hash: String,
    pub output_schema_hash: String,
    pub eval_suite: String,
    pub static_prefix_stable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    pub provider_profile_id: String,
    pub prompt_asset_version: String,
    pub cache_key: String,
    pub hit: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalizationProfile {
    pub id: String,
    pub audience: String,
    pub tone: String,
    pub locale: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerSessionAdapter {
    pub provider: ProviderKind,
    pub session_slot: String,
    pub portability_limited: bool,
    pub deterministic_ci_allowed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CacheRecord {
    cache_key: String,
    provider_profile_id: String,
    prompt_asset_id: String,
    prompt_version: String,
    personalization_profile_id: String,
    model: String,
    adapter_type: String,
    response_text: String,
    recorded_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProvenanceRecord {
    provenance_id: String,
    provider_profile_id: String,
    provider_kind: ProviderKind,
    prompt_asset_id: String,
    prompt_version: String,
    personalization_profile_id: String,
    model: String,
    adapter_type: String,
    experimental_mode: ExperimentalGenerationMode,
    cache_key: String,
    recorded_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EncryptedStateRecord {
    algorithm: String,
    scope: String,
    nonce_b64: String,
    ciphertext_b64: String,
    recorded_at: String,
}

pub fn catalog_json() -> Value {
    let providers: Vec<_> = provider_profiles()
        .into_iter()
        .map(|profile| profile.id)
        .collect();
    let prompt_assets: Vec<_> = prompt_assets().into_iter().map(|asset| asset.id).collect();
    let personalization_profiles: Vec<_> = personalization_profiles()
        .into_iter()
        .map(|profile| profile.id)
        .collect();

    json!({
        "providers": providers,
        "modes": ["prompt-versioned", "personalized", "consumer-session"],
        "promptAssets": prompt_assets,
        "personalizationProfiles": personalization_profiles,
        "flags": [
            "experimental-provider",
            "experimental-mode",
            "experimental-profile",
            "experimental-prompt-asset",
            "experimental-prompt-version",
            "experimental-personalization-profile",
            "experimental-model",
            "experimental-base-url",
            "experimental-session-file",
            "experimental-store",
            "experimental-bootstrap-command",
            "experimental-disable-cache"
        ]
    })
}

pub fn run(config: &SessionConfig, sequence: &mut u64) -> Result<Vec<EventEnvelope>, String> {
    let experimental = config.experimental.as_ref().ok_or_else(|| {
        "experimental runtime requires explicit provider configuration".to_string()
    })?;

    let provider_profile = resolve_provider_profile(experimental)?;
    let prompt_asset = resolve_prompt_asset(experimental)?;
    let prompt_version = resolve_prompt_version(experimental, &prompt_asset)?;
    let personalization = resolve_personalization_profile(experimental)?;
    let store_root = store_root(experimental);

    ensure_store_dirs(&store_root)?;

    let cache_key = build_cache_key(
        config,
        experimental,
        &provider_profile,
        &prompt_asset,
        &prompt_version,
        &personalization,
    );
    let prompt_text = build_prompt(config, &prompt_asset, &prompt_version, &personalization);

    let mut events = Vec::new();

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "session.start",
        format!(
            "starting guarded experimental session via {}",
            provider_profile.id
        ),
        None,
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("project", config.project_name.clone()),
            ("cacheKey", cache_key.clone()),
        ]),
    ));

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "provider.profile.loaded",
        format!("loaded provider profile {}", provider_profile.id),
        Some(GeneratorFamily::AiInferenceOps),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("providerProfileId", provider_profile.id.clone()),
            (
                "providerKind",
                provider_label(provider_profile.provider_kind),
            ),
            (
                "adapterType",
                adapter_label(provider_profile.adapter_type).to_string(),
            ),
            (
                "baseUrl",
                provider_profile
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "local-only".to_string()),
            ),
        ]),
    ));

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "prompt.asset.loaded",
        format!(
            "resolved prompt asset {}@{}",
            prompt_asset.id,
            prompt_version
                .clone()
                .unwrap_or_else(|| "unversioned".to_string())
        ),
        Some(GeneratorFamily::EvaluationAndGuardrails),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("promptAssetId", prompt_asset.id.clone()),
            (
                "promptVersion",
                prompt_version
                    .clone()
                    .unwrap_or_else(|| "unversioned".to_string()),
            ),
        ]),
    ));

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "personalization.loaded",
        format!("loaded personalization profile {}", personalization.id),
        Some(GeneratorFamily::AibomProvenance),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("personalizationProfileId", personalization.id.clone()),
            ("locale", personalization.locale.clone()),
            ("tone", personalization.tone.clone()),
        ]),
    ));

    if !experimental.disable_cache {
        if let Some(cache_record) = read_cache_record(&store_root, &cache_key)? {
            *sequence += 1;
            events.push(experimental_event(
                *sequence,
                "cache.status",
                format!("cache hit for {}", provider_profile.id),
                Some(GeneratorFamily::AibomProvenance),
                &provider_profile,
                prompt_version.as_deref(),
                experimental,
                context_map(&[
                    ("cacheKey", cache_key.clone()),
                    ("cacheHit", "true".to_string()),
                    ("recordedAt", cache_record.recorded_at.clone()),
                ]),
            ));

            *sequence += 1;
            events.push(experimental_event(
                *sequence,
                "provider.response.received",
                truncate_message(&cache_record.response_text),
                Some(GeneratorFamily::AiInferenceOps),
                &provider_profile,
                prompt_version.as_deref(),
                experimental,
                context_map(&[
                    ("cacheKey", cache_key.clone()),
                    ("cacheHit", "true".to_string()),
                    ("providerProfileId", provider_profile.id.clone()),
                ]),
            ));

            *sequence += 1;
            events.push(experimental_event(
                *sequence,
                "session.end",
                "session terminated (experimental-cache-hit)".to_string(),
                None,
                &provider_profile,
                prompt_version.as_deref(),
                experimental,
                context_map(&[("reason", "experimental-cache-hit".to_string())]),
            ));

            return Ok(events);
        }
    }

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "cache.status",
        format!("cache miss for {}", provider_profile.id),
        Some(GeneratorFamily::AibomProvenance),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("cacheKey", cache_key.clone()),
            ("cacheHit", "false".to_string()),
        ]),
    ));

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "provider.request.dispatched",
        format!(
            "dispatching guarded live request via {}",
            provider_profile.id
        ),
        Some(GeneratorFamily::AiInferenceOps),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("cacheKey", cache_key.clone()),
            ("mode", mode_label(experimental.mode).to_string()),
        ]),
    ));

    let response_text = match provider_profile.provider_kind {
        ProviderKind::LocalDemo => {
            render_local_demo_response(config, &provider_profile, &prompt_asset, &personalization)
        }
        ProviderKind::OpenAiCompatibleApi => {
            invoke_openai_compatible(experimental, &provider_profile, &prompt_text)?
        }
        ProviderKind::AnthropicApi => {
            invoke_anthropic(experimental, &provider_profile, &prompt_text)?
        }
        ProviderKind::OpenAiConsumer | ProviderKind::ClaudeConsumer => {
            let imported = load_consumer_session_material(experimental)?;
            persist_encrypted_consumer_state(
                &store_root,
                &provider_profile,
                &prompt_asset,
                prompt_version.as_deref(),
                &personalization,
                &imported,
            )?;

            *sequence += 1;
            events.push(experimental_event(
                *sequence,
                "consumer.session.loaded",
                format!(
                    "loaded guarded consumer session material for {}",
                    provider_profile.id
                ),
                Some(GeneratorFamily::AgentWorkflows),
                &provider_profile,
                prompt_version.as_deref(),
                experimental,
                context_map(&[
                    ("providerProfileId", provider_profile.id.clone()),
                    ("materialSource", imported.source.clone()),
                ]),
            ));

            if imported.bootstrap_command.is_some() {
                *sequence += 1;
                events.push(experimental_event(
                    *sequence,
                    "consumer.session.bootstrap",
                    format!("captured bootstrap command for {}", provider_profile.id),
                    Some(GeneratorFamily::EdgeClientRuntime),
                    &provider_profile,
                    prompt_version.as_deref(),
                    experimental,
                    context_map(&[
                        ("providerProfileId", provider_profile.id.clone()),
                        ("bootstrapMode", "env-or-cli".to_string()),
                    ]),
                ));
            }

            extract_consumer_response(&imported.material)
        }
    };

    let cache_record = CacheRecord {
        cache_key: cache_key.clone(),
        provider_profile_id: provider_profile.id.clone(),
        prompt_asset_id: prompt_asset.id.clone(),
        prompt_version: prompt_version
            .clone()
            .unwrap_or_else(|| "unversioned".to_string()),
        personalization_profile_id: personalization.id.clone(),
        model: effective_model(experimental, &provider_profile),
        adapter_type: adapter_label(provider_profile.adapter_type).to_string(),
        response_text: response_text.clone(),
        recorded_at: now_label(),
    };

    let provenance = ProvenanceRecord {
        provenance_id: short_hash(&format!(
            "{}:{}:{}",
            cache_key, cache_record.model, cache_record.recorded_at
        )),
        provider_profile_id: provider_profile.id.clone(),
        provider_kind: provider_profile.provider_kind,
        prompt_asset_id: prompt_asset.id.clone(),
        prompt_version: prompt_version
            .clone()
            .unwrap_or_else(|| "unversioned".to_string()),
        personalization_profile_id: personalization.id.clone(),
        model: cache_record.model.clone(),
        adapter_type: cache_record.adapter_type.clone(),
        experimental_mode: experimental.mode,
        cache_key: cache_key.clone(),
        recorded_at: cache_record.recorded_at.clone(),
    };

    if !experimental.disable_cache {
        write_cache_record(&store_root, &cache_record)?;
    }
    write_provenance_record(&store_root, &provenance)?;

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "provider.response.received",
        truncate_message(&response_text),
        Some(GeneratorFamily::AiInferenceOps),
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[
            ("cacheKey", cache_key.clone()),
            ("cacheHit", "false".to_string()),
            ("providerProfileId", provider_profile.id.clone()),
            ("provenanceId", provenance.provenance_id.clone()),
        ]),
    ));

    *sequence += 1;
    events.push(experimental_event(
        *sequence,
        "session.end",
        "session terminated (experimental-complete)".to_string(),
        None,
        &provider_profile,
        prompt_version.as_deref(),
        experimental,
        context_map(&[("reason", "experimental-complete".to_string())]),
    ));

    Ok(events)
}

fn provider_profiles() -> Vec<ProviderProfile> {
    vec![
        ProviderProfile {
            id: "local-demo".to_string(),
            provider_kind: ProviderKind::LocalDemo,
            base_url: None,
            model: "stakeholder-local-demo".to_string(),
            adapter_type: AdapterType::LocalDemo,
            prompt_caching: true,
        },
        ProviderProfile {
            id: "openai-compatible".to_string(),
            provider_kind: ProviderKind::OpenAiCompatibleApi,
            base_url: Some("https://api.openai.com".to_string()),
            model: "openai-compatible-model".to_string(),
            adapter_type: AdapterType::Api,
            prompt_caching: true,
        },
        ProviderProfile {
            id: "anthropic".to_string(),
            provider_kind: ProviderKind::AnthropicApi,
            base_url: Some("https://api.anthropic.com".to_string()),
            model: "anthropic-model".to_string(),
            adapter_type: AdapterType::Api,
            prompt_caching: true,
        },
        ProviderProfile {
            id: "openai-consumer".to_string(),
            provider_kind: ProviderKind::OpenAiConsumer,
            base_url: None,
            model: "consumer-session".to_string(),
            adapter_type: AdapterType::Consumer,
            prompt_caching: false,
        },
        ProviderProfile {
            id: "claude-consumer".to_string(),
            provider_kind: ProviderKind::ClaudeConsumer,
            base_url: None,
            model: "consumer-session".to_string(),
            adapter_type: AdapterType::Consumer,
            prompt_caching: false,
        },
    ]
}

fn prompt_assets() -> Vec<PromptAsset> {
    vec![
        PromptAsset {
            id: "stakeholder-live-brief".to_string(),
            owner: "stakeholder-core".to_string(),
            description: "Live provider brief for stakeholder session generation".to_string(),
        },
        PromptAsset {
            id: "consumer-replay-brief".to_string(),
            owner: "stakeholder-core".to_string(),
            description: "Consumer session replay summary prompt".to_string(),
        },
    ]
}

fn prompt_asset_versions() -> Vec<PromptAssetVersion> {
    vec![
        PromptAssetVersion {
            prompt_asset_id: "stakeholder-live-brief".to_string(),
            version: "2026.04.13".to_string(),
            tool_schema_hash: short_hash("stakeholder-live-brief-tool-schema"),
            output_schema_hash: short_hash("stakeholder-live-brief-output-schema"),
            eval_suite: "experimental-live-provider-smoke".to_string(),
            static_prefix_stable: true,
        },
        PromptAssetVersion {
            prompt_asset_id: "consumer-replay-brief".to_string(),
            version: "2026.04.13".to_string(),
            tool_schema_hash: short_hash("consumer-replay-brief-tool-schema"),
            output_schema_hash: short_hash("consumer-replay-brief-output-schema"),
            eval_suite: "experimental-live-provider-smoke".to_string(),
            static_prefix_stable: true,
        },
    ]
}

fn personalization_profiles() -> Vec<PersonalizationProfile> {
    vec![PersonalizationProfile {
        id: "local-operator".to_string(),
        audience: "operator".to_string(),
        tone: "direct".to_string(),
        locale: "en-US".to_string(),
    }]
}

fn consumer_session_adapters() -> Vec<ConsumerSessionAdapter> {
    vec![
        ConsumerSessionAdapter {
            provider: ProviderKind::OpenAiConsumer,
            session_slot: "openai-browser".to_string(),
            portability_limited: true,
            deterministic_ci_allowed: false,
        },
        ConsumerSessionAdapter {
            provider: ProviderKind::ClaudeConsumer,
            session_slot: "claude-browser".to_string(),
            portability_limited: true,
            deterministic_ci_allowed: false,
        },
    ]
}

fn resolve_provider_profile(experimental: &ExperimentalConfig) -> Result<ProviderProfile, String> {
    let profiles = provider_profiles();

    if let Some(profile_id) = experimental.provider_profile.as_ref() {
        if let Some(profile) = profiles
            .into_iter()
            .find(|profile| &profile.id == profile_id)
        {
            return Ok(profile);
        }

        return Err(format!(
            "unknown experimental provider profile: {profile_id}"
        ));
    }

    profiles
        .into_iter()
        .find(|profile| profile.provider_kind == experimental.provider)
        .ok_or_else(|| "no provider profile available for requested provider".to_string())
}

fn resolve_prompt_asset(experimental: &ExperimentalConfig) -> Result<PromptAsset, String> {
    let requested = experimental
        .prompt_asset
        .clone()
        .unwrap_or_else(|| default_prompt_asset_id(experimental.provider).to_string());

    prompt_assets()
        .into_iter()
        .find(|asset| asset.id == requested)
        .ok_or_else(|| format!("unknown prompt asset: {requested}"))
}

fn resolve_prompt_version(
    experimental: &ExperimentalConfig,
    prompt_asset: &PromptAsset,
) -> Result<Option<String>, String> {
    let requested = experimental
        .prompt_version
        .clone()
        .unwrap_or_else(|| "2026.04.13".to_string());

    prompt_asset_versions()
        .into_iter()
        .find(|version| version.prompt_asset_id == prompt_asset.id && version.version == requested)
        .map(|version| Some(version.version))
        .ok_or_else(|| {
            format!(
                "unknown prompt version {} for {}",
                requested, prompt_asset.id
            )
        })
}

fn resolve_personalization_profile(
    experimental: &ExperimentalConfig,
) -> Result<PersonalizationProfile, String> {
    let requested = experimental
        .personalization_profile
        .clone()
        .unwrap_or_else(|| "local-operator".to_string());

    personalization_profiles()
        .into_iter()
        .find(|profile| profile.id == requested)
        .ok_or_else(|| format!("unknown personalization profile: {requested}"))
}

fn build_prompt(
    config: &SessionConfig,
    prompt_asset: &PromptAsset,
    prompt_version: &Option<String>,
    personalization: &PersonalizationProfile,
) -> String {
    format!(
        "asset={} version={} audience={} tone={} locale={} project={} dev_type={} framework={} complexity={} jargon={:?}. Produce a concise stakeholder-facing live runtime update with explicit provenance and cache awareness.",
        prompt_asset.id,
        prompt_version
            .clone()
            .unwrap_or_else(|| "unversioned".to_string()),
        personalization.audience,
        personalization.tone,
        personalization.locale,
        config.project_name,
        serde_json::to_string(&config.dev_type)
            .unwrap_or_else(|_| "\"unknown\"".to_string())
            .replace('\"', ""),
        if config.framework.is_empty() {
            "none".to_string()
        } else {
            config.framework.clone()
        },
        config.complexity.activity_count(),
        config.jargon_level,
    )
}

fn invoke_openai_compatible(
    experimental: &ExperimentalConfig,
    provider_profile: &ProviderProfile,
    prompt_text: &str,
) -> Result<String, String> {
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY is required for openai-compatible experimental mode".to_string()
    })?;
    let model = experimental
        .model
        .clone()
        .or_else(|| std::env::var("OPENAI_MODEL").ok())
        .unwrap_or_else(|| provider_profile.model.clone());
    let base_url = experimental
        .base_url
        .clone()
        .or_else(|| std::env::var("OPENAI_BASE_URL").ok())
        .or_else(|| provider_profile.base_url.clone())
        .ok_or_else(|| "missing base URL for openai-compatible profile".to_string())?;

    let endpoint = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));
    let response = Client::new()
        .post(endpoint)
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "messages": [
                { "role": "user", "content": prompt_text }
            ]
        }))
        .send()
        .map_err(|err| format!("openai-compatible request failed: {err}"))?
        .error_for_status()
        .map_err(|err| format!("openai-compatible response failed: {err}"))?
        .json::<Value>()
        .map_err(|err| format!("openai-compatible JSON decode failed: {err}"))?;

    extract_provider_text(&response)
        .ok_or_else(|| "openai-compatible response did not include readable text".to_string())
}

fn invoke_anthropic(
    experimental: &ExperimentalConfig,
    provider_profile: &ProviderProfile,
    prompt_text: &str,
) -> Result<String, String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY is required for anthropic experimental mode".to_string())?;
    let model = experimental
        .model
        .clone()
        .or_else(|| std::env::var("ANTHROPIC_MODEL").ok())
        .unwrap_or_else(|| provider_profile.model.clone());
    let base_url = experimental
        .base_url
        .clone()
        .or_else(|| provider_profile.base_url.clone())
        .ok_or_else(|| "missing base URL for anthropic profile".to_string())?;

    let endpoint = format!("{}/v1/messages", base_url.trim_end_matches('/'));
    let response = Client::new()
        .post(endpoint)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 256,
            "messages": [
                { "role": "user", "content": prompt_text }
            ]
        }))
        .send()
        .map_err(|err| format!("anthropic request failed: {err}"))?
        .error_for_status()
        .map_err(|err| format!("anthropic response failed: {err}"))?
        .json::<Value>()
        .map_err(|err| format!("anthropic JSON decode failed: {err}"))?;

    extract_provider_text(&response)
        .ok_or_else(|| "anthropic response did not include readable text".to_string())
}

fn render_local_demo_response(
    config: &SessionConfig,
    provider_profile: &ProviderProfile,
    prompt_asset: &PromptAsset,
    personalization: &PersonalizationProfile,
) -> String {
    format!(
        "local demo response via {} for project {} using {} and {} tone; cache/provenance path is active and deterministic fixtures remain untouched",
        provider_profile.id, config.project_name, prompt_asset.id, personalization.tone
    )
}

struct ImportedConsumerSession {
    material: String,
    source: String,
    bootstrap_command: Option<String>,
}

fn load_consumer_session_material(
    experimental: &ExperimentalConfig,
) -> Result<ImportedConsumerSession, String> {
    let bootstrap_command = experimental
        .bootstrap_command
        .clone()
        .or_else(|| std::env::var("STAKEHOLDER_BROWSER_BOOTSTRAP_CMD").ok());

    if let Some(path) = experimental.session_file.as_ref() {
        let material = fs::read_to_string(path)
            .map_err(|err| format!("failed to read experimental session file {}: {}", path, err))?;
        return Ok(ImportedConsumerSession {
            material,
            source: format!("session-file:{path}"),
            bootstrap_command,
        });
    }

    if let Some(command) = bootstrap_command.clone() {
        return Ok(ImportedConsumerSession {
            material: json!({
                "bootstrapCommand": command,
                "responseText": "consumer-session replay synthesized from bootstrap command metadata"
            })
            .to_string(),
            source: "bootstrap-command".to_string(),
            bootstrap_command: Some(command),
        });
    }

    Err("consumer-session providers require --experimental-session-file or STAKEHOLDER_BROWSER_BOOTSTRAP_CMD".to_string())
}

fn persist_encrypted_consumer_state(
    store_root: &Path,
    provider_profile: &ProviderProfile,
    prompt_asset: &PromptAsset,
    prompt_version: Option<&str>,
    personalization: &PersonalizationProfile,
    imported: &ImportedConsumerSession,
) -> Result<(), String> {
    let key = std::env::var("STAKEHOLDER_EXPERIMENTAL_STORE_KEY")
        .map_err(|_| "consumer-session providers require STAKEHOLDER_EXPERIMENTAL_STORE_KEY for encrypted local persistence".to_string())?;
    let plaintext = json!({
        "providerProfileId": provider_profile.id,
        "promptAssetId": prompt_asset.id,
        "promptVersion": prompt_version.unwrap_or("unversioned"),
        "personalizationProfileId": personalization.id,
        "materialSource": imported.source,
        "bootstrapCommand": imported.bootstrap_command,
        "material": imported.material,
    })
    .to_string();

    let encrypted = encrypt_record("consumer-session", &plaintext, &key)?;
    let secret_path = secret_record_path(store_root, &provider_profile.id);
    fs::write(
        secret_path,
        serde_json::to_vec_pretty(&encrypted)
            .map_err(|err| format!("failed to serialize encrypted state: {err}"))?,
    )
    .map_err(|err| format!("failed to persist encrypted consumer state: {err}"))
}

fn read_cache_record(store_root: &Path, cache_key: &str) -> Result<Option<CacheRecord>, String> {
    let path = cache_record_path(store_root, cache_key);
    if !path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&path)
        .map_err(|err| format!("failed to read cache record {}: {}", path.display(), err))?;
    serde_json::from_str(&contents)
        .map(Some)
        .map_err(|err| format!("failed to decode cache record {}: {}", path.display(), err))
}

fn write_cache_record(store_root: &Path, record: &CacheRecord) -> Result<(), String> {
    fs::write(
        cache_record_path(store_root, &record.cache_key),
        serde_json::to_vec_pretty(record)
            .map_err(|err| format!("failed to encode cache record: {err}"))?,
    )
    .map_err(|err| format!("failed to write cache record: {err}"))
}

fn write_provenance_record(store_root: &Path, record: &ProvenanceRecord) -> Result<(), String> {
    fs::write(
        provenance_record_path(store_root, &record.cache_key),
        serde_json::to_vec_pretty(record)
            .map_err(|err| format!("failed to encode provenance record: {err}"))?,
    )
    .map_err(|err| format!("failed to write provenance record: {err}"))
}

fn ensure_store_dirs(store_root: &Path) -> Result<(), String> {
    fs::create_dir_all(store_root.join("cache"))
        .and_then(|_| fs::create_dir_all(store_root.join("provenance")))
        .and_then(|_| fs::create_dir_all(store_root.join("secrets")))
        .map_err(|err| {
            format!(
                "failed to prepare experimental store {}: {}",
                store_root.display(),
                err
            )
        })
}

fn store_root(experimental: &ExperimentalConfig) -> PathBuf {
    experimental
        .store_path
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".stakeholder/experimental"))
}

fn cache_record_path(store_root: &Path, cache_key: &str) -> PathBuf {
    store_root.join("cache").join(format!("{cache_key}.json"))
}

fn provenance_record_path(store_root: &Path, cache_key: &str) -> PathBuf {
    store_root
        .join("provenance")
        .join(format!("{cache_key}.json"))
}

fn secret_record_path(store_root: &Path, provider_profile_id: &str) -> PathBuf {
    store_root
        .join("secrets")
        .join(format!("{provider_profile_id}.json"))
}

fn encrypt_record(
    scope: &str,
    plaintext: &str,
    key_material: &str,
) -> Result<EncryptedStateRecord, String> {
    let key = Sha256::digest(key_material.as_bytes());
    let cipher = Aes256GcmSiv::new_from_slice(&key)
        .map_err(|err| format!("failed to initialize encrypted store cipher: {err}"))?;

    let mut nonce_bytes = [0_u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|err| format!("failed to encrypt local provider state: {err}"))?;

    Ok(EncryptedStateRecord {
        algorithm: "aes-256-gcm-siv".to_string(),
        scope: scope.to_string(),
        nonce_b64: BASE64.encode(nonce_bytes),
        ciphertext_b64: BASE64.encode(ciphertext),
        recorded_at: now_label(),
    })
}

fn build_cache_key(
    config: &SessionConfig,
    experimental: &ExperimentalConfig,
    provider_profile: &ProviderProfile,
    prompt_asset: &PromptAsset,
    prompt_version: &Option<String>,
    personalization: &PersonalizationProfile,
) -> String {
    short_hash(&format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        config.project_name,
        serde_json::to_string(&config.dev_type).unwrap_or_else(|_| "\"unknown\"".to_string()),
        provider_profile.id,
        provider_label(provider_profile.provider_kind),
        prompt_asset.id,
        prompt_version
            .clone()
            .unwrap_or_else(|| "unversioned".to_string()),
        personalization.id,
        effective_model(experimental, provider_profile),
        config.framework,
        config.complexity.activity_count(),
        mode_label(experimental.mode),
    ))
}

fn experimental_event(
    sequence: u64,
    event_type: &str,
    message: String,
    family: Option<GeneratorFamily>,
    provider_profile: &ProviderProfile,
    prompt_version: Option<&str>,
    _experimental: &ExperimentalConfig,
    context: BTreeMap<String, String>,
) -> EventEnvelope {
    EventEnvelope {
        event_type: event_type.to_string(),
        sequence,
        timestamp: format!("T+{:06}ms", sequence * 137),
        message,
        family,
        protocol: None,
        schema_ref: None,
        flavors: vec![],
        generation_provenance: GenerationProvenance {
            source_repo: "rust-stakeholder".to_string(),
            baseline: "2026-plus-source-evolution".to_string(),
            experimental: true,
            adapter_type: adapter_label(provider_profile.adapter_type).to_string(),
            prompt_version: prompt_version.map(|value| value.to_string()),
        },
        context,
    }
}

fn context_map(entries: &[(&str, String)]) -> BTreeMap<String, String> {
    let mut context = BTreeMap::new();
    for (key, value) in entries {
        context.insert((*key).to_string(), value.clone());
    }
    context
}

fn effective_model(
    experimental: &ExperimentalConfig,
    provider_profile: &ProviderProfile,
) -> String {
    experimental
        .model
        .clone()
        .unwrap_or_else(|| provider_profile.model.clone())
}

fn provider_label(provider: ProviderKind) -> String {
    match provider {
        ProviderKind::LocalDemo => "local-demo",
        ProviderKind::OpenAiCompatibleApi => "openai-compatible",
        ProviderKind::AnthropicApi => "anthropic",
        ProviderKind::OpenAiConsumer => "openai-consumer",
        ProviderKind::ClaudeConsumer => "claude-consumer",
    }
    .to_string()
}

fn mode_label(mode: ExperimentalGenerationMode) -> &'static str {
    match mode {
        ExperimentalGenerationMode::PromptVersioned => "prompt-versioned",
        ExperimentalGenerationMode::Personalized => "personalized",
        ExperimentalGenerationMode::ConsumerSession => "consumer-session",
    }
}

fn adapter_label(adapter: AdapterType) -> &'static str {
    match adapter {
        AdapterType::LocalDemo => "local-demo",
        AdapterType::Api => "api",
        AdapterType::Consumer => "consumer",
    }
}

fn default_prompt_asset_id(provider: ProviderKind) -> &'static str {
    match provider {
        ProviderKind::OpenAiConsumer | ProviderKind::ClaudeConsumer => "consumer-replay-brief",
        _ => "stakeholder-live-brief",
    }
}

fn short_hash(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn now_label() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("unix:{}", duration.as_secs()),
        Err(_) => "unix:0".to_string(),
    }
}

fn truncate_message(message: &str) -> String {
    const LIMIT: usize = 180;
    if message.chars().count() <= LIMIT {
        return message.to_string();
    }

    let truncated: String = message.chars().take(LIMIT).collect();
    format!("{truncated}...")
}

fn extract_consumer_response(material: &str) -> String {
    if let Ok(json) = serde_json::from_str::<Value>(material) {
        if let Some(value) = extract_provider_text(&json) {
            return value;
        }
    }

    truncate_message(material)
}

fn extract_provider_text(value: &Value) -> Option<String> {
    for pointer in [
        "/responseText",
        "/response/text",
        "/choices/0/message/content",
        "/content/0/text",
        "/messages/0/content",
        "/output_text",
        "/output/0/content/0/text",
    ] {
        if let Some(candidate) = value.pointer(pointer) {
            if let Some(text) = value_to_text(candidate) {
                return Some(text);
            }
        }
    }

    None
}

fn value_to_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Array(items) => {
            let collected: Vec<_> = items.iter().filter_map(value_to_text).collect();
            if collected.is_empty() {
                None
            } else {
                Some(collected.join(" "))
            }
        }
        Value::Object(map) => map
            .get("text")
            .and_then(value_to_text)
            .or_else(|| map.get("content").and_then(value_to_text)),
        _ => None,
    }
}
