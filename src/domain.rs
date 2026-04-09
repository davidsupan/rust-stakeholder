use crate::types::{LanguagePack, SecurityPersona};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorFamily {
    CodeAnalyzer,
    DataProcessing,
    Jargon,
    Metrics,
    NetworkActivity,
    SystemMonitoring,
    AgentWorkflows,
    AiInferenceOps,
    PlatformEngineering,
    SupplyChainSecurity,
    ObservabilityAiRuntime,
    DeliveryPreviewOps,
    EvaluationAndGuardrails,
    KnowledgeRetrieval,
    EdgeClientRuntime,
    IdentityAndTrust,
    AibomProvenance,
    AgentBoundarySecurity,
    EmbeddedAgenticPipeline,
    DataGovernanceCompliance,
    FinopsCapacity,
    BlockchainProtocolOps,
    CrossChainInterop,
    ProofAndSequencerOps,
    HybridRuntimeOps,
    CapacityCostController,
    BatchExecutionTuner,
    CompilerMaintainer,
    InteropAdapterEngineer,
    PreflightCapacityPlanner,
    SimulatorPerformanceEngineer,
    FhirProfileGenerator,
    SmartLaunchOauth,
    BulkFhirPopulationOps,
    Hl7v2FeedOps,
    ClinicalWorkflowEvents,
    DicomwebImagingOps,
    OpenehrSemanticRecordOps,
    DeviceTelemetryClinical,
    EmrVendorAdapter,
    OcppChargepointOps,
    OcpiRoamingOps,
    McpA2aOps,
    StreamingBusOps,
    ServiceMeshRpcOps,
}

pub const ALL_GENERATOR_FAMILIES: [GeneratorFamily; 45] = [
    GeneratorFamily::CodeAnalyzer,
    GeneratorFamily::DataProcessing,
    GeneratorFamily::Jargon,
    GeneratorFamily::Metrics,
    GeneratorFamily::NetworkActivity,
    GeneratorFamily::SystemMonitoring,
    GeneratorFamily::AgentWorkflows,
    GeneratorFamily::AiInferenceOps,
    GeneratorFamily::PlatformEngineering,
    GeneratorFamily::SupplyChainSecurity,
    GeneratorFamily::ObservabilityAiRuntime,
    GeneratorFamily::DeliveryPreviewOps,
    GeneratorFamily::EvaluationAndGuardrails,
    GeneratorFamily::KnowledgeRetrieval,
    GeneratorFamily::EdgeClientRuntime,
    GeneratorFamily::IdentityAndTrust,
    GeneratorFamily::AibomProvenance,
    GeneratorFamily::AgentBoundarySecurity,
    GeneratorFamily::EmbeddedAgenticPipeline,
    GeneratorFamily::DataGovernanceCompliance,
    GeneratorFamily::FinopsCapacity,
    GeneratorFamily::BlockchainProtocolOps,
    GeneratorFamily::CrossChainInterop,
    GeneratorFamily::ProofAndSequencerOps,
    GeneratorFamily::HybridRuntimeOps,
    GeneratorFamily::CapacityCostController,
    GeneratorFamily::BatchExecutionTuner,
    GeneratorFamily::CompilerMaintainer,
    GeneratorFamily::InteropAdapterEngineer,
    GeneratorFamily::PreflightCapacityPlanner,
    GeneratorFamily::SimulatorPerformanceEngineer,
    GeneratorFamily::FhirProfileGenerator,
    GeneratorFamily::SmartLaunchOauth,
    GeneratorFamily::BulkFhirPopulationOps,
    GeneratorFamily::Hl7v2FeedOps,
    GeneratorFamily::ClinicalWorkflowEvents,
    GeneratorFamily::DicomwebImagingOps,
    GeneratorFamily::OpenehrSemanticRecordOps,
    GeneratorFamily::DeviceTelemetryClinical,
    GeneratorFamily::EmrVendorAdapter,
    GeneratorFamily::OcppChargepointOps,
    GeneratorFamily::OcpiRoamingOps,
    GeneratorFamily::McpA2aOps,
    GeneratorFamily::StreamingBusOps,
    GeneratorFamily::ServiceMeshRpcOps,
];

impl GeneratorFamily {
    pub fn label(self) -> &'static str {
        match self {
            Self::CodeAnalyzer => "code-analyzer",
            Self::DataProcessing => "data-processing",
            Self::Jargon => "jargon",
            Self::Metrics => "metrics",
            Self::NetworkActivity => "network-activity",
            Self::SystemMonitoring => "system-monitoring",
            Self::AgentWorkflows => "agent-workflows",
            Self::AiInferenceOps => "ai-inference-ops",
            Self::PlatformEngineering => "platform-engineering",
            Self::SupplyChainSecurity => "supply-chain-security",
            Self::ObservabilityAiRuntime => "observability-ai-runtime",
            Self::DeliveryPreviewOps => "delivery-preview-ops",
            Self::EvaluationAndGuardrails => "evaluation-and-guardrails",
            Self::KnowledgeRetrieval => "knowledge-retrieval",
            Self::EdgeClientRuntime => "edge-client-runtime",
            Self::IdentityAndTrust => "identity-and-trust",
            Self::AibomProvenance => "aibom-provenance",
            Self::AgentBoundarySecurity => "agent-boundary-security",
            Self::EmbeddedAgenticPipeline => "embedded-agentic-pipeline",
            Self::DataGovernanceCompliance => "data-governance-compliance",
            Self::FinopsCapacity => "finops-capacity",
            Self::BlockchainProtocolOps => "blockchain-protocol-ops",
            Self::CrossChainInterop => "cross-chain-interop",
            Self::ProofAndSequencerOps => "proof-and-sequencer-ops",
            Self::HybridRuntimeOps => "hybrid-runtime-ops",
            Self::CapacityCostController => "capacity-cost-controller",
            Self::BatchExecutionTuner => "batch-execution-tuner",
            Self::CompilerMaintainer => "compiler-maintainer",
            Self::InteropAdapterEngineer => "interop-adapter-engineer",
            Self::PreflightCapacityPlanner => "preflight-capacity-planner",
            Self::SimulatorPerformanceEngineer => "simulator-performance-engineer",
            Self::FhirProfileGenerator => "fhir-profile-generator",
            Self::SmartLaunchOauth => "smart-launch-oauth",
            Self::BulkFhirPopulationOps => "bulk-fhir-population-ops",
            Self::Hl7v2FeedOps => "hl7v2-feed-ops",
            Self::ClinicalWorkflowEvents => "clinical-workflow-events",
            Self::DicomwebImagingOps => "dicomweb-imaging-ops",
            Self::OpenehrSemanticRecordOps => "openehr-semantic-record-ops",
            Self::DeviceTelemetryClinical => "device-telemetry-clinical",
            Self::EmrVendorAdapter => "emr-vendor-adapter",
            Self::OcppChargepointOps => "ocpp-chargepoint-ops",
            Self::OcpiRoamingOps => "ocpi-roaming-ops",
            Self::McpA2aOps => "mcp-a2a-ops",
            Self::StreamingBusOps => "streaming-bus-ops",
            Self::ServiceMeshRpcOps => "service-mesh-rpc-ops",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    Generator(GeneratorFamily),
    AlertInjection,
    TeamInjection,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioFlavor {
    MultilingualSecurity(LanguagePack),
    SecurityPersona(SecurityPersona),
    ExperimentalLiveProvider,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolAdapter {
    ResponsesApi,
    AnthropicMessages,
    Mcp,
    A2a,
    Mqtt5,
    NatsJetStream,
    Kafka,
    Grpc,
    GraphqlFederation,
    WebTransport,
    OpenQasm3,
    Qir,
    FhirR4,
    FhirR5,
    SmartLaunch,
    BulkFhir,
    Hl7v2,
    Dicomweb,
    OpenEhr,
    IheDevice,
    EpicFhir,
    OracleHealth,
    Ocpp16,
    Ocpp201,
    Ocpp21,
    Ocpi2x,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRef {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthContext {
    pub scheme: String,
    pub principal: String,
    pub scope: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    pub strategy: String,
    pub max_attempts: u8,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdempotencyKey {
    pub scope: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationProvenance {
    pub source_repo: String,
    pub baseline: String,
    pub experimental: bool,
    pub adapter_type: String,
    pub prompt_version: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandEnvelope {
    pub family: GeneratorFamily,
    pub protocol: Option<ProtocolAdapter>,
    pub schema_ref: Option<SchemaRef>,
    pub auth_context: Option<AuthContext>,
    pub retry_policy: RetryPolicy,
    pub idempotency_key: Option<IdempotencyKey>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventEnvelope {
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub sequence: u64,
    pub timestamp: String,
    pub message: String,
    pub family: Option<GeneratorFamily>,
    pub protocol: Option<ProtocolAdapter>,
    pub schema_ref: Option<SchemaRef>,
    pub flavors: Vec<ScenarioFlavor>,
    pub generation_provenance: GenerationProvenance,
    pub context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySelection {
    pub kind: ActivityKind,
    pub family: GeneratorFamily,
    pub flavors: Vec<ScenarioFlavor>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPlan {
    pub activities: Vec<ActivitySelection>,
}
