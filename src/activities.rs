use crate::config::SessionConfig;
use crate::domain::{
    ActivityKind, ActivityPlan, ActivitySelection, EventEnvelope, GenerationProvenance,
    GeneratorFamily, ScenarioFlavor, ALL_GENERATOR_FAMILIES,
};
use crate::generators;
use crate::types::{DevelopmentType, JargonLevel, LanguagePack, SecurityPersona};
use colored::Colorize;
use rand::{prelude::IndexedRandom, rngs::StdRng};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

const CLASSIC_FAMILIES: [GeneratorFamily; 5] = [
    GeneratorFamily::CodeAnalyzer,
    GeneratorFamily::DataProcessing,
    GeneratorFamily::Metrics,
    GeneratorFamily::NetworkActivity,
    GeneratorFamily::SystemMonitoring,
];

const POLICY_FAMILIES: [GeneratorFamily; 8] = [
    GeneratorFamily::SupplyChainSecurity,
    GeneratorFamily::ObservabilityAiRuntime,
    GeneratorFamily::EvaluationAndGuardrails,
    GeneratorFamily::IdentityAndTrust,
    GeneratorFamily::AibomProvenance,
    GeneratorFamily::AgentBoundarySecurity,
    GeneratorFamily::DataGovernanceCompliance,
    GeneratorFamily::FinopsCapacity,
];

const ALERT_FAMILIES: [GeneratorFamily; 8] = [
    GeneratorFamily::SupplyChainSecurity,
    GeneratorFamily::ObservabilityAiRuntime,
    GeneratorFamily::AgentBoundarySecurity,
    GeneratorFamily::DeviceTelemetryClinical,
    GeneratorFamily::OcppChargepointOps,
    GeneratorFamily::StreamingBusOps,
    GeneratorFamily::ServiceMeshRpcOps,
    GeneratorFamily::McpA2aOps,
];

const TEAM_FAMILIES: [GeneratorFamily; 4] = [
    GeneratorFamily::AgentWorkflows,
    GeneratorFamily::PlatformEngineering,
    GeneratorFamily::DeliveryPreviewOps,
    GeneratorFamily::ServiceMeshRpcOps,
];

pub fn intro_lines(config: &SessionConfig) -> Vec<String> {
    let mut lines = vec![
        format!("2026+ source-evolution session for {}", config.project_name),
        format!(
            "mode={} jargon={:?} complexity={} output={:?}",
            serde_json::to_string(&config.dev_type)
                .unwrap_or_else(|_| "\"unknown\"".to_string())
                .replace('"', ""),
            config.jargon_level,
            config.complexity.activity_count(),
            config.output_format,
        ),
    ];

    if let Some(seed) = config.seed {
        lines.push(format!("seed={seed}"));
    }
    if !config.framework.is_empty() {
        lines.push(format!("framework={}", config.framework));
    }
    if config.alerts_enabled {
        lines.push("alerts=enabled".to_string());
    }
    if config.team_activity {
        lines.push("team=enabled".to_string());
    }

    lines
}

pub fn list_values_json() -> serde_json::Value {
    let families: Vec<_> = ALL_GENERATOR_FAMILIES
        .iter()
        .map(|family| family.label())
        .collect();
    json!({
        "devTypes": [
            "backend",
            "frontend",
            "fullstack",
            "data_science",
            "dev_ops",
            "blockchain",
            "machine_learning",
            "systems_programming",
            "game_development",
            "security"
        ],
        "jargonLevels": ["low", "medium", "high", "extreme"],
        "complexities": ["low", "medium", "high", "extreme"],
        "outputFormats": ["text", "json"],
        "flags": [
            "alerts",
            "project",
            "minimal",
            "team",
            "framework",
            "seed",
            "output-format",
            "no-color",
            "trace",
            "list-values"
        ],
        "generatorFamilies": families
    })
}

pub fn boot_event(config: &SessionConfig, sequence: u64) -> EventEnvelope {
    let mut context = BTreeMap::new();
    context.insert("project".to_string(), config.project_name.clone());
    context.insert(
        "devType".to_string(),
        serde_json::to_string(&config.dev_type)
            .unwrap_or_else(|_| "\"unknown\"".to_string())
            .replace('"', ""),
    );
    if let Some(seed) = config.seed {
        context.insert("seed".to_string(), seed.to_string());
    }
    if !config.framework.is_empty() {
        context.insert("framework".to_string(), config.framework.clone());
    }

    EventEnvelope {
        event_type: "session.start".to_string(),
        sequence,
        timestamp: format!("T+{:06}ms", sequence * 137),
        message: format!(
            "starting 2026+ source-evolution session for {}",
            config.project_name
        ),
        family: None,
        protocol: None,
        schema_ref: None,
        flavors: vec![],
        generation_provenance: GenerationProvenance {
            source_repo: "rust-stakeholder".to_string(),
            baseline: "2026-plus-source-evolution".to_string(),
            experimental: false,
            adapter_type: "static-catalog".to_string(),
            prompt_version: None,
        },
        context,
    }
}

pub fn termination_event(sequence: u64, reason: &str) -> EventEnvelope {
    let mut context = BTreeMap::new();
    context.insert("reason".to_string(), reason.to_string());

    EventEnvelope {
        event_type: "session.end".to_string(),
        sequence,
        timestamp: format!("T+{:06}ms", sequence * 137),
        message: format!("session terminated ({reason})"),
        family: None,
        protocol: None,
        schema_ref: None,
        flavors: vec![],
        generation_provenance: GenerationProvenance {
            source_repo: "rust-stakeholder".to_string(),
            baseline: "2026-plus-source-evolution".to_string(),
            experimental: false,
            adapter_type: "static-catalog".to_string(),
            prompt_version: None,
        },
        context,
    }
}

pub fn emit_cycle(
    config: &SessionConfig,
    rng: &mut StdRng,
    sequence: &mut u64,
) -> Vec<EventEnvelope> {
    let plan = build_activity_plan(config, rng);
    let mut events = Vec::new();

    for selection in plan.activities {
        *sequence += 1;
        events.push(generators::render_activity(
            selection.family,
            config,
            rng,
            *sequence,
            &selection.flavors,
        ));

        if config.trace_enabled {
            *sequence += 1;
            events.push(trace_event(*sequence, &selection));
        }
    }

    events
}

pub fn print_text_event(config: &SessionConfig, event: &EventEnvelope) {
    let base = if let Some(family) = event.family {
        format!(
            "[{}] {}",
            generators::title_for_family(family),
            event.message
        )
    } else {
        event.message.clone()
    };

    if config.minimal_output || config.no_color {
        println!("{base}");
        return;
    }

    let rendered = match event.family {
        Some(family) if is_health_family(family) => base.bright_green().to_string(),
        Some(family) if is_quantum_family(family) => base.bright_cyan().to_string(),
        Some(family) if is_blockchain_family(family) => base.bright_magenta().to_string(),
        Some(family) if is_security_related(family) => base.bright_red().to_string(),
        Some(family) if is_protocol_family(family) => base.bright_blue().to_string(),
        Some(_) => base.bright_white().to_string(),
        None => base.bold().bright_yellow().to_string(),
    };

    println!("{rendered}");
}

fn trace_event(sequence: u64, selection: &ActivitySelection) -> EventEnvelope {
    let mut context = BTreeMap::new();
    context.insert("family".to_string(), selection.family.label().to_string());
    if let Some(protocol) = generators::protocol_for_family(selection.family) {
        context.insert(
            "protocolAdapter".to_string(),
            serde_json::to_string(&protocol)
                .unwrap_or_else(|_| "\"unknown\"".to_string())
                .replace('"', ""),
        );
    }

    EventEnvelope {
        event_type: "trace".to_string(),
        sequence,
        timestamp: format!("T+{:06}ms", sequence * 137),
        message: format!(
            "scheduled {} with {} flavor(s)",
            selection.family.label(),
            selection.flavors.len()
        ),
        family: Some(selection.family),
        protocol: generators::protocol_for_family(selection.family),
        schema_ref: generators::schema_ref_for_family(selection.family),
        flavors: selection.flavors.clone(),
        generation_provenance: GenerationProvenance {
            source_repo: "rust-stakeholder".to_string(),
            baseline: "2026-plus-source-evolution".to_string(),
            experimental: false,
            adapter_type: "trace".to_string(),
            prompt_version: None,
        },
        context,
    }
}

fn build_activity_plan(config: &SessionConfig, rng: &mut StdRng) -> ActivityPlan {
    let target_count = config.complexity.activity_count();
    let eligible = eligible_families(config);
    let mut selected: Vec<GeneratorFamily> = Vec::new();

    push_unique_from_pool(&mut selected, &eligible, &CLASSIC_FAMILIES, rng);

    if target_count >= 2 {
        let modern: Vec<_> = eligible
            .iter()
            .copied()
            .filter(|family| {
                !CLASSIC_FAMILIES.contains(family) && *family != GeneratorFamily::Jargon
            })
            .collect();
        push_unique_from_vec(&mut selected, &modern, rng);
    }

    if target_count >= 3 {
        push_unique_from_pool(&mut selected, &eligible, &POLICY_FAMILIES, rng);
    }

    while selected.len() < target_count {
        push_unique_from_vec(&mut selected, &eligible, rng);
    }

    if config.alerts_enabled {
        push_unique_from_pool(&mut selected, &eligible, &ALERT_FAMILIES, rng);
    }

    if config.team_activity {
        push_unique_from_pool(&mut selected, &eligible, &TEAM_FAMILIES, rng);
    }

    let activities = selected
        .into_iter()
        .map(|family| ActivitySelection {
            kind: if config.alerts_enabled && ALERT_FAMILIES.contains(&family) {
                ActivityKind::AlertInjection
            } else if config.team_activity && TEAM_FAMILIES.contains(&family) {
                ActivityKind::TeamInjection
            } else {
                ActivityKind::Generator(family)
            },
            family,
            flavors: resolve_flavors(config, family, rng),
        })
        .collect();

    ActivityPlan { activities }
}

fn eligible_families(config: &SessionConfig) -> Vec<GeneratorFamily> {
    let mut set = BTreeSet::new();
    for family in CLASSIC_FAMILIES {
        set.insert(family);
    }

    match config.dev_type {
        DevelopmentType::Backend => extend(
            &mut set,
            &[
                GeneratorFamily::AgentWorkflows,
                GeneratorFamily::AiInferenceOps,
                GeneratorFamily::PlatformEngineering,
                GeneratorFamily::SupplyChainSecurity,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::DeliveryPreviewOps,
                GeneratorFamily::EvaluationAndGuardrails,
                GeneratorFamily::KnowledgeRetrieval,
                GeneratorFamily::IdentityAndTrust,
                GeneratorFamily::AibomProvenance,
                GeneratorFamily::DataGovernanceCompliance,
                GeneratorFamily::FinopsCapacity,
                GeneratorFamily::McpA2aOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        ),
        DevelopmentType::Frontend => extend(
            &mut set,
            &[
                GeneratorFamily::AgentWorkflows,
                GeneratorFamily::DeliveryPreviewOps,
                GeneratorFamily::EdgeClientRuntime,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::KnowledgeRetrieval,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        ),
        DevelopmentType::Fullstack => extend(
            &mut set,
            &[
                GeneratorFamily::AgentWorkflows,
                GeneratorFamily::AiInferenceOps,
                GeneratorFamily::PlatformEngineering,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::DeliveryPreviewOps,
                GeneratorFamily::KnowledgeRetrieval,
                GeneratorFamily::McpA2aOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
                GeneratorFamily::SupplyChainSecurity,
            ],
        ),
        DevelopmentType::DataScience => extend(
            &mut set,
            &[
                GeneratorFamily::AiInferenceOps,
                GeneratorFamily::KnowledgeRetrieval,
                GeneratorFamily::EvaluationAndGuardrails,
                GeneratorFamily::AibomProvenance,
                GeneratorFamily::DataGovernanceCompliance,
                GeneratorFamily::ObservabilityAiRuntime,
            ],
        ),
        DevelopmentType::DevOps => extend(
            &mut set,
            &[
                GeneratorFamily::AgentWorkflows,
                GeneratorFamily::PlatformEngineering,
                GeneratorFamily::SupplyChainSecurity,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::DeliveryPreviewOps,
                GeneratorFamily::IdentityAndTrust,
                GeneratorFamily::FinopsCapacity,
                GeneratorFamily::McpA2aOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        ),
        DevelopmentType::Blockchain => extend(
            &mut set,
            &[
                GeneratorFamily::BlockchainProtocolOps,
                GeneratorFamily::CrossChainInterop,
                GeneratorFamily::ProofAndSequencerOps,
                GeneratorFamily::SupplyChainSecurity,
                GeneratorFamily::IdentityAndTrust,
                GeneratorFamily::McpA2aOps,
            ],
        ),
        DevelopmentType::MachineLearning => extend(
            &mut set,
            &[
                GeneratorFamily::AiInferenceOps,
                GeneratorFamily::KnowledgeRetrieval,
                GeneratorFamily::EvaluationAndGuardrails,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::AibomProvenance,
                GeneratorFamily::FinopsCapacity,
            ],
        ),
        DevelopmentType::SystemsProgramming => extend(
            &mut set,
            &[
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::EmbeddedAgenticPipeline,
                GeneratorFamily::IdentityAndTrust,
                GeneratorFamily::SupplyChainSecurity,
                GeneratorFamily::StreamingBusOps,
            ],
        ),
        DevelopmentType::GameDevelopment => extend(
            &mut set,
            &[
                GeneratorFamily::EdgeClientRuntime,
                GeneratorFamily::DeliveryPreviewOps,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        ),
        DevelopmentType::Security => extend(
            &mut set,
            &[
                GeneratorFamily::AgentWorkflows,
                GeneratorFamily::SupplyChainSecurity,
                GeneratorFamily::ObservabilityAiRuntime,
                GeneratorFamily::EvaluationAndGuardrails,
                GeneratorFamily::IdentityAndTrust,
                GeneratorFamily::AibomProvenance,
                GeneratorFamily::AgentBoundarySecurity,
                GeneratorFamily::DataGovernanceCompliance,
                GeneratorFamily::McpA2aOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        ),
    }

    let context_text = format!(
        "{} {}",
        config.project_name.to_lowercase(),
        config.framework.to_lowercase()
    );

    if contains_keyword(
        &context_text,
        &[
            "ehr", "emr", "fhir", "hl7", "openehr", "dicom", "clinical", "patient", "hospital",
        ],
    ) {
        extend(
            &mut set,
            &[
                GeneratorFamily::FhirProfileGenerator,
                GeneratorFamily::SmartLaunchOauth,
                GeneratorFamily::BulkFhirPopulationOps,
                GeneratorFamily::Hl7v2FeedOps,
                GeneratorFamily::ClinicalWorkflowEvents,
                GeneratorFamily::DicomwebImagingOps,
                GeneratorFamily::OpenehrSemanticRecordOps,
                GeneratorFamily::DeviceTelemetryClinical,
                GeneratorFamily::EmrVendorAdapter,
            ],
        );
    }

    if contains_keyword(
        &context_text,
        &[
            "charge", "charger", "charging", "ev", "ocpp", "ocpi", "roaming",
        ],
    ) {
        extend(
            &mut set,
            &[
                GeneratorFamily::OcppChargepointOps,
                GeneratorFamily::OcpiRoamingOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        );
    }

    if contains_keyword(
        &context_text,
        &[
            "quantum", "qir", "qasm", "braket", "qiskit", "cudaq", "ionq",
        ],
    ) {
        extend(
            &mut set,
            &[
                GeneratorFamily::HybridRuntimeOps,
                GeneratorFamily::CapacityCostController,
                GeneratorFamily::BatchExecutionTuner,
                GeneratorFamily::CompilerMaintainer,
                GeneratorFamily::InteropAdapterEngineer,
                GeneratorFamily::PreflightCapacityPlanner,
                GeneratorFamily::SimulatorPerformanceEngineer,
            ],
        );
    }

    if contains_keyword(
        &context_text,
        &[
            "mcp",
            "a2a",
            "mqtt",
            "nats",
            "kafka",
            "grpc",
            "graphql",
            "webtransport",
        ],
    ) {
        extend(
            &mut set,
            &[
                GeneratorFamily::McpA2aOps,
                GeneratorFamily::StreamingBusOps,
                GeneratorFamily::ServiceMeshRpcOps,
            ],
        );
    }

    set.into_iter().collect()
}

fn resolve_flavors(
    config: &SessionConfig,
    family: GeneratorFamily,
    rng: &mut StdRng,
) -> Vec<ScenarioFlavor> {
    let mut flavors = Vec::new();
    if config.dev_type == DevelopmentType::Security || is_security_related(family) {
        if config.jargon_level >= JargonLevel::High || config.alerts_enabled {
            let languages = [
                LanguagePack::English,
                LanguagePack::Chinese,
                LanguagePack::Russian,
                LanguagePack::Spanish,
                LanguagePack::Arabic,
            ];
            if let Some(language) = languages.choose(rng) {
                flavors.push(ScenarioFlavor::MultilingualSecurity(*language));
            }
        }
        if config.jargon_level >= JargonLevel::High {
            let personas = [
                SecurityPersona::BugBountyOperator,
                SecurityPersona::IncidentCommander,
                SecurityPersona::ReverseEngineer,
                SecurityPersona::ThreatHunter,
                SecurityPersona::SocAnalyst,
                SecurityPersona::DarkMarketWatcher,
                SecurityPersona::CtiBriefWriter,
            ];
            if let Some(persona) = personas.choose(rng) {
                flavors.push(ScenarioFlavor::SecurityPersona(*persona));
            }
        }
    }

    if contains_keyword(
        &format!(
            "{} {}",
            config.project_name.to_lowercase(),
            config.framework.to_lowercase()
        ),
        &[
            "experimental",
            "openai",
            "anthropic",
            "claude",
            "responses",
            "llm",
        ],
    ) && matches!(
        family,
        GeneratorFamily::AiInferenceOps
            | GeneratorFamily::EvaluationAndGuardrails
            | GeneratorFamily::AibomProvenance
    ) {
        flavors.push(ScenarioFlavor::ExperimentalLiveProvider);
    }

    flavors
}

fn push_unique_from_pool(
    selected: &mut Vec<GeneratorFamily>,
    eligible: &[GeneratorFamily],
    pool: &[GeneratorFamily],
    rng: &mut StdRng,
) {
    let candidates: Vec<_> = pool
        .iter()
        .copied()
        .filter(|family| eligible.contains(family) && !selected.contains(family))
        .collect();
    push_unique_from_vec(selected, &candidates, rng);
}

fn push_unique_from_vec(
    selected: &mut Vec<GeneratorFamily>,
    candidates: &[GeneratorFamily],
    rng: &mut StdRng,
) {
    if let Some(choice) = candidates.choose(rng) {
        if !selected.contains(choice) {
            selected.push(*choice);
        }
    }
}

fn extend(target: &mut BTreeSet<GeneratorFamily>, families: &[GeneratorFamily]) {
    for family in families {
        target.insert(*family);
    }
}

fn contains_keyword(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn is_security_related(family: GeneratorFamily) -> bool {
    matches!(
        family,
        GeneratorFamily::SupplyChainSecurity
            | GeneratorFamily::AgentBoundarySecurity
            | GeneratorFamily::IdentityAndTrust
            | GeneratorFamily::AibomProvenance
            | GeneratorFamily::DataGovernanceCompliance
            | GeneratorFamily::McpA2aOps
            | GeneratorFamily::BlockchainProtocolOps
            | GeneratorFamily::CrossChainInterop
            | GeneratorFamily::ProofAndSequencerOps
    )
}

fn is_blockchain_family(family: GeneratorFamily) -> bool {
    matches!(
        family,
        GeneratorFamily::BlockchainProtocolOps
            | GeneratorFamily::CrossChainInterop
            | GeneratorFamily::ProofAndSequencerOps
    )
}

fn is_quantum_family(family: GeneratorFamily) -> bool {
    matches!(
        family,
        GeneratorFamily::HybridRuntimeOps
            | GeneratorFamily::CapacityCostController
            | GeneratorFamily::BatchExecutionTuner
            | GeneratorFamily::CompilerMaintainer
            | GeneratorFamily::InteropAdapterEngineer
            | GeneratorFamily::PreflightCapacityPlanner
            | GeneratorFamily::SimulatorPerformanceEngineer
    )
}

fn is_health_family(family: GeneratorFamily) -> bool {
    matches!(
        family,
        GeneratorFamily::FhirProfileGenerator
            | GeneratorFamily::SmartLaunchOauth
            | GeneratorFamily::BulkFhirPopulationOps
            | GeneratorFamily::Hl7v2FeedOps
            | GeneratorFamily::ClinicalWorkflowEvents
            | GeneratorFamily::DicomwebImagingOps
            | GeneratorFamily::OpenehrSemanticRecordOps
            | GeneratorFamily::DeviceTelemetryClinical
            | GeneratorFamily::EmrVendorAdapter
    )
}

fn is_protocol_family(family: GeneratorFamily) -> bool {
    matches!(
        family,
        GeneratorFamily::OcppChargepointOps
            | GeneratorFamily::OcpiRoamingOps
            | GeneratorFamily::McpA2aOps
            | GeneratorFamily::StreamingBusOps
            | GeneratorFamily::ServiceMeshRpcOps
            | GeneratorFamily::NetworkActivity
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Complexity, OutputFormat};
    use rand::SeedableRng;

    fn test_config() -> SessionConfig {
        SessionConfig {
            dev_type: DevelopmentType::Security,
            jargon_level: JargonLevel::High,
            complexity: Complexity::Extreme,
            alerts_enabled: true,
            project_name: "hospital-ocpp-quantum-control".to_string(),
            minimal_output: true,
            team_activity: true,
            framework: "mcp grpc".to_string(),
            seed: Some(7),
            output_format: OutputFormat::Json,
            no_color: true,
            trace_enabled: true,
        }
    }

    #[test]
    fn plan_is_deterministic_for_the_same_seed() {
        let config = test_config();
        let mut rng_a = StdRng::seed_from_u64(41);
        let mut rng_b = StdRng::seed_from_u64(41);

        let plan_a: Vec<_> = build_activity_plan(&config, &mut rng_a)
            .activities
            .into_iter()
            .map(|selection| selection.family)
            .collect();
        let plan_b: Vec<_> = build_activity_plan(&config, &mut rng_b)
            .activities
            .into_iter()
            .map(|selection| selection.family)
            .collect();

        assert_eq!(plan_a, plan_b);
    }

    #[test]
    fn keyword_routing_includes_health_ev_and_quantum_families() {
        let config = test_config();
        let eligible = eligible_families(&config);

        assert!(eligible.contains(&GeneratorFamily::FhirProfileGenerator));
        assert!(eligible.contains(&GeneratorFamily::OcppChargepointOps));
        assert!(eligible.contains(&GeneratorFamily::HybridRuntimeOps));
    }

    #[test]
    fn security_flavors_are_applied_to_security_families() {
        let config = test_config();
        let mut rng = StdRng::seed_from_u64(9);
        let flavors = resolve_flavors(&config, GeneratorFamily::SupplyChainSecurity, &mut rng);

        assert!(!flavors.is_empty());
        assert!(
            flavors
                .iter()
                .any(|flavor| matches!(flavor, ScenarioFlavor::MultilingualSecurity(_)))
        );
        assert!(
            flavors
                .iter()
                .any(|flavor| matches!(flavor, ScenarioFlavor::SecurityPersona(_)))
        );
    }
}
