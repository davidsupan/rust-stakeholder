use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    OpenAiCompatibleApi,
    AnthropicApi,
    OpenAiConsumer,
    ClaudeConsumer,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterType {
    Api,
    Consumer,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentalGenerationMode {
    Disabled,
    PromptVersioned,
    Personalized,
    ConsumerSession,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderProfile {
    pub id: String,
    pub provider_kind: ProviderKind,
    pub base_url: Option<String>,
    pub model: String,
    pub adapter_type: AdapterType,
    pub prompt_caching: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAsset {
    pub id: String,
    pub owner: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAssetVersion {
    pub prompt_asset_id: String,
    pub version: String,
    pub tool_schema_hash: String,
    pub output_schema_hash: String,
    pub eval_suite: String,
    pub static_prefix_stable: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    pub provider_profile_id: String,
    pub prompt_asset_version: String,
    pub cache_key: String,
    pub hit: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalizationProfile {
    pub id: String,
    pub audience: String,
    pub tone: String,
    pub locale: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerSessionAdapter {
    pub provider: ProviderKind,
    pub session_slot: String,
    pub portability_limited: bool,
    pub deterministic_ci_allowed: bool,
}
