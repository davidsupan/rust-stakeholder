use crate::config::SessionConfig;
use crate::domain::{EventEnvelope, GeneratorFamily, ProtocolAdapter, ScenarioFlavor, SchemaRef};
use rand::rngs::StdRng;

pub mod agent_boundary_security;
pub mod agent_workflows;
pub mod ai_inference_ops;
pub mod aibom_provenance;
pub mod batch_execution_tuner;
pub mod blockchain_protocol_ops;
pub mod bulk_fhir_population_ops;
pub mod capacity_cost_controller;
pub mod clinical_workflow_events;
pub mod code_analyzer;
pub mod common;
pub mod compiler_maintainer;
pub mod cross_chain_interop;
pub mod data_governance_compliance;
pub mod data_processing;
pub mod delivery_preview_ops;
pub mod device_telemetry_clinical;
pub mod dicomweb_imaging_ops;
pub mod edge_client_runtime;
pub mod embedded_agentic_pipeline;
pub mod emr_vendor_adapter;
pub mod evaluation_and_guardrails;
pub mod fhir_profile_generator;
pub mod finops_capacity;
pub mod hl7v2_feed_ops;
pub mod hybrid_runtime_ops;
pub mod identity_and_trust;
pub mod interop_adapter_engineer;
pub mod jargon;
pub mod knowledge_retrieval;
pub mod mcp_a2a_ops;
pub mod metrics;
pub mod network_activity;
pub mod observability_ai_runtime;
pub mod ocpi_roaming_ops;
pub mod ocpp_chargepoint_ops;
pub mod openehr_semantic_record_ops;
pub mod platform_engineering;
pub mod preflight_capacity_planner;
pub mod proof_and_sequencer_ops;
pub mod service_mesh_rpc_ops;
pub mod simulator_performance_engineer;
pub mod smart_launch_oauth;
pub mod streaming_bus_ops;
pub mod supply_chain_security;
pub mod system_monitoring;

pub fn title_for_family(family: GeneratorFamily) -> &'static str {
    common::title_for_family(family)
}

pub fn protocol_for_family(family: GeneratorFamily) -> Option<ProtocolAdapter> {
    common::protocol_for_family(family)
}

pub fn schema_ref_for_family(family: GeneratorFamily) -> Option<SchemaRef> {
    common::schema_ref_for_family(family)
}

pub fn render_activity(
    family: GeneratorFamily,
    config: &SessionConfig,
    rng: &mut StdRng,
    sequence: u64,
    flavors: &[ScenarioFlavor],
) -> EventEnvelope {
    match family {
        GeneratorFamily::CodeAnalyzer => code_analyzer::render(config, rng, sequence, flavors),
        GeneratorFamily::DataProcessing => data_processing::render(config, rng, sequence, flavors),
        GeneratorFamily::Jargon => jargon::render(config, rng, sequence, flavors),
        GeneratorFamily::Metrics => metrics::render(config, rng, sequence, flavors),
        GeneratorFamily::NetworkActivity => {
            network_activity::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::SystemMonitoring => {
            system_monitoring::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::AgentWorkflows => agent_workflows::render(config, rng, sequence, flavors),
        GeneratorFamily::AiInferenceOps => ai_inference_ops::render(config, rng, sequence, flavors),
        GeneratorFamily::PlatformEngineering => {
            platform_engineering::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::SupplyChainSecurity => {
            supply_chain_security::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::ObservabilityAiRuntime => {
            observability_ai_runtime::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::DeliveryPreviewOps => {
            delivery_preview_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::EvaluationAndGuardrails => {
            evaluation_and_guardrails::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::KnowledgeRetrieval => {
            knowledge_retrieval::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::EdgeClientRuntime => {
            edge_client_runtime::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::IdentityAndTrust => {
            identity_and_trust::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::AibomProvenance => {
            aibom_provenance::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::AgentBoundarySecurity => {
            agent_boundary_security::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::EmbeddedAgenticPipeline => {
            embedded_agentic_pipeline::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::DataGovernanceCompliance => {
            data_governance_compliance::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::FinopsCapacity => finops_capacity::render(config, rng, sequence, flavors),
        GeneratorFamily::BlockchainProtocolOps => {
            blockchain_protocol_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::CrossChainInterop => {
            cross_chain_interop::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::ProofAndSequencerOps => {
            proof_and_sequencer_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::HybridRuntimeOps => {
            hybrid_runtime_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::CapacityCostController => {
            capacity_cost_controller::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::BatchExecutionTuner => {
            batch_execution_tuner::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::CompilerMaintainer => {
            compiler_maintainer::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::InteropAdapterEngineer => {
            interop_adapter_engineer::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::PreflightCapacityPlanner => {
            preflight_capacity_planner::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::SimulatorPerformanceEngineer => {
            simulator_performance_engineer::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::FhirProfileGenerator => {
            fhir_profile_generator::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::SmartLaunchOauth => {
            smart_launch_oauth::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::BulkFhirPopulationOps => {
            bulk_fhir_population_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::Hl7v2FeedOps => hl7v2_feed_ops::render(config, rng, sequence, flavors),
        GeneratorFamily::ClinicalWorkflowEvents => {
            clinical_workflow_events::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::DicomwebImagingOps => {
            dicomweb_imaging_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::OpenehrSemanticRecordOps => {
            openehr_semantic_record_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::DeviceTelemetryClinical => {
            device_telemetry_clinical::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::EmrVendorAdapter => {
            emr_vendor_adapter::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::OcppChargepointOps => {
            ocpp_chargepoint_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::OcpiRoamingOps => ocpi_roaming_ops::render(config, rng, sequence, flavors),
        GeneratorFamily::McpA2aOps => mcp_a2a_ops::render(config, rng, sequence, flavors),
        GeneratorFamily::StreamingBusOps => {
            streaming_bus_ops::render(config, rng, sequence, flavors)
        }
        GeneratorFamily::ServiceMeshRpcOps => {
            service_mesh_rpc_ops::render(config, rng, sequence, flavors)
        }
    }
}
