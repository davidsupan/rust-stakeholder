use crate::config::SessionConfig;
use crate::domain::{
    EventEnvelope, GenerationProvenance, GeneratorFamily, ProtocolAdapter, ScenarioFlavor,
    SchemaRef,
};
use crate::types::JargonLevel;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
struct Descriptor {
    title: &'static str,
    protocol: Option<ProtocolAdapter>,
    schema: Option<(&'static str, &'static str)>,
    low: &'static str,
    high: &'static str,
    extreme: &'static str,
}

fn descriptor(family: GeneratorFamily) -> Descriptor {
    match family {
        GeneratorFamily::CodeAnalyzer => Descriptor {
            title: "Code analyzer",
            protocol: None,
            schema: None,
            low: "reviewing typed interfaces and SDK drift across the active service graph",
            high: "triaging monorepo dependency edges, generated patches, and schema compatibility before merge",
            extreme: "replaying agent-authored patchsets against contract drift, ownership boundaries, and MCP tool assumptions",
        },
        GeneratorFamily::DataProcessing => Descriptor {
            title: "Data processing",
            protocol: None,
            schema: None,
            low: "refreshing embedding corpora, batch transforms, and event windows for the current dataset",
            high: "rebuilding hybrid retrieval indexes, semantic chunks, and NDJSON backfills for downstream consumers",
            extreme: "reconciling multimodal pipelines, lakehouse batch cuts, and evaluation-ready data slices under deterministic ordering",
        },
        GeneratorFamily::Jargon => Descriptor {
            title: "Jargon refresh",
            protocol: None,
            schema: None,
            low: "keeping technical language current without drifting into fake-deep jargon",
            high: "switching phrasing toward credible 2026 agent, platform, protocol, and security terminology",
            extreme: "enforcing modern domain vocabulary so advanced output stays precise instead of sounding synthetic",
        },
        GeneratorFamily::Metrics => Descriptor {
            title: "Metrics",
            protocol: None,
            schema: None,
            low: "tracking queue depth, latency bands, and cost signals across the active workload",
            high: "correlating token spend, SLO burn, GPU occupancy, and attestation coverage in one metrics lane",
            extreme: "folding evaluation score movement, blob economics, and runner pressure into a single operations dashboard",
        },
        GeneratorFamily::NetworkActivity => Descriptor {
            title: "Network activity",
            protocol: Some(ProtocolAdapter::Grpc),
            schema: None,
            low: "observing RPC, event-stream, and adapter traffic across the current service boundary",
            high: "mapping MCP calls, inference APIs, registry fetches, and cross-domain message flow under backpressure",
            extreme: "profiling mixed gRPC, Kafka, MQTT, and bridge traffic while preserving replay semantics and retry windows",
        },
        GeneratorFamily::SystemMonitoring => Descriptor {
            title: "System monitoring",
            protocol: None,
            schema: None,
            low: "watching collector pressure, runner health, and process saturation on the active stack",
            high: "capturing GPU memory pressure, secret-scan spikes, sandbox failures, and scheduler queue churn",
            extreme: "stitching host telemetry, proof queues, provisioning lag, and policy denials into one operational heartbeat",
        },
        GeneratorFamily::AgentWorkflows => Descriptor {
            title: "Agent workflows",
            protocol: Some(ProtocolAdapter::Mcp),
            schema: Some(("agent-workflow-envelope", "2026-04")),
            low: "routing coding-agent work through review queues and approval gates",
            high: "coordinating delegated patch runs, blocked tool calls, and human checkpoints across multiple repos",
            extreme: "orchestrating branch handoff envelopes, MCP leases, and merge-safe approval chains for background agents",
        },
        GeneratorFamily::AiInferenceOps => Descriptor {
            title: "AI inference ops",
            protocol: Some(ProtocolAdapter::ResponsesApi),
            schema: Some(("inference-event", "2026-04")),
            low: "monitoring model routing, cache hits, and prompt rollouts for live inference paths",
            high: "tuning fallback chains, context-budget pressure, and eval regressions across provider tiers",
            extreme: "balancing retrieval freshness, safety fallbacks, and token-cost envelopes under multi-model orchestration",
        },
        GeneratorFamily::PlatformEngineering => Descriptor {
            title: "Platform engineering",
            protocol: None,
            schema: None,
            low: "maintaining golden paths, service templates, and workload identity for self-service delivery",
            high: "resolving platform policy denials, tenant quotas, and template drift inside the internal developer portal",
            extreme: "reconciling workload identity, cluster tenancy, policy bundles, and queue fairness across platform control planes",
        },
        GeneratorFamily::SupplyChainSecurity => Descriptor {
            title: "Supply-chain security",
            protocol: None,
            schema: Some(("provenance-check", "2026-04")),
            low: "checking artifact trust, secret exposure, and dependency health before release",
            high: "verifying provenance attestations, AIBOM coverage, revocation posture, and tamper signals across build lanes",
            extreme: "gating release promotion on signed artifacts, dependency substitution checks, and cross-tool trust evidence",
        },
        GeneratorFamily::ObservabilityAiRuntime => Descriptor {
            title: "Observability AI runtime",
            protocol: None,
            schema: Some(("otel-runtime-event", "2026-04")),
            low: "recording traces, token spend, and latency bands for the active runtime",
            high: "tracking OTel collector saturation, span cardinality, and GPU telemetry alongside tool-call traces",
            extreme: "driving burn-rate analysis across inference queues, cost attribution, and distributed reasoning spans",
        },
        GeneratorFamily::DeliveryPreviewOps => Descriptor {
            title: "Delivery preview ops",
            protocol: None,
            schema: None,
            low: "managing preview environments, feature flags, and canary promotions for current changes",
            high: "holding rollout gates on runner saturation, preview drift, and canary health regression signals",
            extreme: "sequencing flag freezes, rollback windows, and staged promotion rules across agent-authored delivery pipelines",
        },
        GeneratorFamily::EvaluationAndGuardrails => Descriptor {
            title: "Evaluation and guardrails",
            protocol: Some(ProtocolAdapter::ResponsesApi),
            schema: Some(("eval-result", "2026-04")),
            low: "running evaluation suites and schema checks against generated outputs",
            high: "measuring tool-use regressions, rubric drift, and structured-output failures before release",
            extreme: "enforcing guardrail coverage against jailbreak attempts, policy escapes, and benchmark regressions in one pass",
        },
        GeneratorFamily::KnowledgeRetrieval => Descriptor {
            title: "Knowledge retrieval",
            protocol: Some(ProtocolAdapter::ResponsesApi),
            schema: Some(("retrieval-run", "2026-04")),
            low: "refreshing vector search indexes and citation coverage for current knowledge sets",
            high: "repairing stale embeddings, reranker drift, and low-confidence retrieval before answers ship",
            extreme: "rebalancing hybrid search, chunk overlap, and provenance coverage under corpus freshness pressure",
        },
        GeneratorFamily::EdgeClientRuntime => Descriptor {
            title: "Edge client runtime",
            protocol: Some(ProtocolAdapter::WebTransport),
            schema: None,
            low: "stabilizing edge execution, streaming UI, and offline sync paths",
            high: "handling hydration boundaries, wasm loads, and client cache invalidation across distributed edges",
            extreme: "coordinating edge cold-start budgets, sync conflict recovery, and realtime streaming under browser constraints",
        },
        GeneratorFamily::IdentityAndTrust => Descriptor {
            title: "Identity and trust",
            protocol: None,
            schema: Some(("trust-context", "2026-04")),
            low: "maintaining signer trust, workload identity, and delegated access boundaries",
            high: "rotating keys, validating session trust, and reconciling workload provenance across service edges",
            extreme: "stitching smart-account recovery, signer provenance, and delegated authority windows into one trust fabric",
        },
        GeneratorFamily::AibomProvenance => Descriptor {
            title: "AIBOM provenance",
            protocol: None,
            schema: Some(("aibom-record", "2026-04")),
            low: "capturing model lineage and runtime dependency provenance for generated outputs",
            high: "versioning prompt assets, adapter state, and AIBOM evidence with reproducible cache metadata",
            extreme: "reconstructing full generation provenance across model lineage, adapter drift, and environment evidence",
        },
        GeneratorFamily::AgentBoundarySecurity => Descriptor {
            title: "Agent boundary security",
            protocol: Some(ProtocolAdapter::Mcp),
            schema: Some(("agent-boundary-alert", "2026-04")),
            low: "checking unsafe delegation, tool overreach, and principal confusion in agent flows",
            high: "blocking retrieval poisoning, denial-of-wallet patterns, and cross-boundary action mistakes",
            extreme: "hardening planning loops against wrong-principal execution, policy bypass, and agent-to-agent trust collapse",
        },
        GeneratorFamily::EmbeddedAgenticPipeline => Descriptor {
            title: "Embedded agentic pipeline",
            protocol: None,
            schema: None,
            low: "keeping deterministic control loops and constrained inference pipelines stable",
            high: "coordinating firmware toolchains, on-device models, and traceable agent steps under resource limits",
            extreme: "balancing safety-critical timing, agentic orchestration, and hardware build provenance across embedded fleets",
        },
        GeneratorFamily::DataGovernanceCompliance => Descriptor {
            title: "Data governance compliance",
            protocol: None,
            schema: Some(("governance-check", "2026-04")),
            low: "applying retention, consent, and regional handling rules to active data flows",
            high: "enforcing governed retrieval, explainability evidence, and audit-ready policy checkpoints",
            extreme: "reconciling cross-border data use, consent state, and explainability artifacts across automated workflows",
        },
        GeneratorFamily::FinopsCapacity => Descriptor {
            title: "FinOps capacity",
            protocol: None,
            schema: None,
            low: "tracking spend, queue pressure, and storage growth across the active platform",
            high: "tuning GPU scheduling, preview-environment budgets, and token-cost ceilings against workload demand",
            extreme: "balancing inference burn, runner economics, and blob-or-data-availability spend under shared capacity limits",
        },
        GeneratorFamily::BlockchainProtocolOps => Descriptor {
            title: "Blockchain protocol ops",
            protocol: None,
            schema: Some(("chain-op", "2026-04")),
            low: "processing rollup, validator, and smart-account operations against the current chain state",
            high: "coordinating gas sponsorship, bridge verification, and sequencer health across modern execution layers",
            extreme: "managing rollup batch flow, validator signals, and smart-account recovery under chain-abstraction demands",
        },
        GeneratorFamily::CrossChainInterop => Descriptor {
            title: "Cross-chain interop",
            protocol: None,
            schema: Some(("interop-flow", "2026-04")),
            low: "routing assets and calls across chain boundaries with explicit trust assumptions",
            high: "coordinating wallet-level chain abstraction, liquidity paths, and cross-domain execution guarantees",
            extreme: "sequencing sign-once cross-chain flows with interoperability proofs, settlement safeguards, and bridge-minimized UX",
        },
        GeneratorFamily::ProofAndSequencerOps => Descriptor {
            title: "Proof and sequencer ops",
            protocol: None,
            schema: Some(("proof-queue", "2026-04")),
            low: "watching proof jobs, sequencing order, and batch submission latency",
            high: "triaging prover queues, ordering policy, MEV pressure, and data-availability throughput",
            extreme: "balancing shared sequencing, proof lag, and finality windows across rollup infrastructure",
        },
        GeneratorFamily::HybridRuntimeOps => Descriptor {
            title: "Hybrid runtime ops",
            protocol: None,
            schema: Some(("hybrid-runtime-job", "2026-04")),
            low: "moving jobs between notebooks, sessions, and managed runtime batches",
            high: "coordinating hybrid execution windows, session reuse, and cancel-or-retry flow on quantum runtimes",
            extreme: "balancing job orchestration across managed sessions, classical sidecars, and backend-specific runtime constraints",
        },
        GeneratorFamily::CapacityCostController => Descriptor {
            title: "Capacity and cost controller",
            protocol: None,
            schema: Some(("capacity-reservation", "2026-04")),
            low: "tracking queue visibility, reservations, and spend controls on quantum backends",
            high: "planning reservations, task admission, and budget alerts against scarce quantum capacity",
            extreme: "holding hybrid workloads inside reservation windows, spend ceilings, and backend admission constraints",
        },
        GeneratorFamily::BatchExecutionTuner => Descriptor {
            title: "Batch execution tuner",
            protocol: None,
            schema: Some(("batch-run", "2026-04")),
            low: "assembling parameter sweeps and batched runs for repeatable throughput checks",
            high: "optimizing batch submission order, parametric compilation reuse, and aggregated result handling",
            extreme: "driving high-volume sweep orchestration across batch windows, compilation reuse, and deterministic result collation",
        },
        GeneratorFamily::CompilerMaintainer => Descriptor {
            title: "Compiler maintainer",
            protocol: Some(ProtocolAdapter::OpenQasm3),
            schema: Some(("transpiler-plan", "2026-04")),
            low: "adjusting transpiler passes and backend targets for the active circuit set",
            high: "managing compiler plugins, routing passes, and backend-target mismatch under current constraints",
            extreme: "tuning custom transpiler stacks, plugin manifests, and pass-manager behavior across backend generations",
        },
        GeneratorFamily::InteropAdapterEngineer => Descriptor {
            title: "Interop adapter engineer",
            protocol: Some(ProtocolAdapter::Qir),
            schema: Some(("qir-bridge", "2026-04")),
            low: "translating execution artifacts across QIR, OpenQASM, and adapter boundaries",
            high: "running round-trip tests between quantum IRs while preserving semantics and backend capability tags",
            extreme: "reconciling QIR adaptor output, OpenQASM transforms, and backend profile gaps under interop pressure",
        },
        GeneratorFamily::PreflightCapacityPlanner => Descriptor {
            title: "Preflight capacity planner",
            protocol: None,
            schema: Some(("capacity-estimate", "2026-04")),
            low: "estimating runtime size, qubit demand, and target fit before dispatch",
            high: "checking backend profiles, resource estimators, and admission limits before quantum submission",
            extreme: "gating workloads on qubit budgets, error tolerance, and backend profile constraints before execution",
        },
        GeneratorFamily::SimulatorPerformanceEngineer => Descriptor {
            title: "Simulator performance engineer",
            protocol: Some(ProtocolAdapter::OpenQasm3),
            schema: None,
            low: "profiling simulator throughput and local-mode performance under current inputs",
            high: "tuning GPU-backed simulators, container runtime compatibility, and local execution benchmarks",
            extreme: "balancing CUDA-class simulation paths, local-mode orchestration, and benchmark repeatability across quantum stacks",
        },
        GeneratorFamily::FhirProfileGenerator => Descriptor {
            title: "FHIR profile generator",
            protocol: Some(ProtocolAdapter::FhirR4),
            schema: Some(("fhir-resource", "r4")),
            low: "generating FHIR R4 resources and profile-constrained clinical records",
            high: "assembling US Core aligned resource graphs, validation rules, and vendor-capability expectations",
            extreme: "producing profile-aware clinical bundles with deployable R4 semantics and forward-looking R5 awareness",
        },
        GeneratorFamily::SmartLaunchOauth => Descriptor {
            title: "SMART launch OAuth",
            protocol: Some(ProtocolAdapter::SmartLaunch),
            schema: Some(("smart-launch-context", "2.2.0")),
            low: "negotiating SMART launch context, scopes, and token refresh for current EHR sessions",
            high: "stabilizing standalone and EHR launch flows with patient, user, and encounter context propagation",
            extreme: "coordinating SMART launch scope negotiation, refresh policy, and context handoff across vendor sandboxes",
        },
        GeneratorFamily::BulkFhirPopulationOps => Descriptor {
            title: "Bulk FHIR population ops",
            protocol: Some(ProtocolAdapter::BulkFhir),
            schema: Some(("bulk-fhir-export", "2.0.0")),
            low: "preparing Bulk FHIR exports and NDJSON population slices",
            high: "running cohort-scale export jobs, manifest tracking, and downstream analytics handoff for current datasets",
            extreme: "coordinating high-volume Bulk Data exports, job polling, and reconciliation against multi-tenant analytics lanes",
        },
        GeneratorFamily::Hl7v2FeedOps => Descriptor {
            title: "HL7 v2 feed ops",
            protocol: Some(ProtocolAdapter::Hl7v2),
            schema: Some(("hl7v2-message", "2.x")),
            low: "processing HL7 v2 feeds, ACKs, and interface-engine traffic for operational workflows",
            high: "mapping ADT, ORM, ORU, and SIU message behavior into modern validation and bridge checks",
            extreme: "reconciling repeating-segment churn, ACK/NACK behavior, and v2-to-FHIR boundary conditions at scale",
        },
        GeneratorFamily::ClinicalWorkflowEvents => Descriptor {
            title: "Clinical workflow events",
            protocol: Some(ProtocolAdapter::FhirR4),
            schema: Some(("clinical-workflow-event", "2026-04")),
            low: "driving CDS Hooks, subscriptions, and workflow triggers for clinical events",
            high: "coordinating prior-auth, questionnaire, measure, and case-reporting workflows across FHIR operations",
            extreme: "orchestrating subscription events, CDS cards, and payer-provider workflow state under live clinical policy",
        },
        GeneratorFamily::DicomwebImagingOps => Descriptor {
            title: "DICOMweb imaging ops",
            protocol: Some(ProtocolAdapter::Dicomweb),
            schema: Some(("dicomweb-study", "2026-04")),
            low: "serving imaging studies over QIDO-RS, WADO-RS, and STOW-RS surfaces",
            high: "matching DICOMweb retrieval, storage, and conformance expectations across modality and PACS flows",
            extreme: "balancing DICOMweb ingest, retrieval, and conformance evidence across imaging workflows and archive tiers",
        },
        GeneratorFamily::OpenehrSemanticRecordOps => Descriptor {
            title: "openEHR semantic record ops",
            protocol: Some(ProtocolAdapter::OpenEhr),
            schema: Some(("openehr-composition", "2026-04")),
            low: "managing openEHR compositions, templates, and semantic record queries",
            high: "running archetype-driven data capture with AQL query validation and template-aware persistence",
            extreme: "reconciling template semantics, composition versioning, and AQL-driven access across longitudinal records",
        },
        GeneratorFamily::DeviceTelemetryClinical => Descriptor {
            title: "Device telemetry clinical",
            protocol: Some(ProtocolAdapter::IheDevice),
            schema: Some(("clinical-device-telemetry", "2026-04")),
            low: "tracking bedside device telemetry, alerts, and identity across care settings",
            high: "coordinating IHE device flows, monitor alarms, and point-of-care telemetry under clinical constraints",
            extreme: "balancing device identity, telemetry burst control, and alert escalation across monitored clinical infrastructure",
        },
        GeneratorFamily::EmrVendorAdapter => Descriptor {
            title: "EMR vendor adapter",
            protocol: Some(ProtocolAdapter::EpicFhir),
            schema: Some(("vendor-capability-matrix", "2026-04")),
            low: "aligning application flows with Epic and Oracle Health integration patterns",
            high: "normalizing vendor-specific launch, scope, and error behavior across EHR adapter boundaries",
            extreme: "reconciling vendor capability gaps, sandbox behavior, and operational error modes across EMR integrations",
        },
        GeneratorFamily::OcppChargepointOps => Descriptor {
            title: "OCPP chargepoint ops",
            protocol: Some(ProtocolAdapter::Ocpp21),
            schema: Some(("ocpp-session", "2.0.1/2.1")),
            low: "handling charger sessions, heartbeats, and smart-charging commands over OCPP",
            high: "coordinating OCPP 2.x profiles, security state, and brownfield 1.6 compatibility boundaries",
            extreme: "balancing certificate flows, smart charging, and transaction lifecycle state across mixed charger fleets",
        },
        GeneratorFamily::OcpiRoamingOps => Descriptor {
            title: "OCPI roaming ops",
            protocol: Some(ProtocolAdapter::Ocpi2x),
            schema: Some(("ocpi-roaming-event", "2.2.1")),
            low: "processing roaming session state, tariffs, and settlement exchanges over OCPI",
            high: "managing CPO-to-EMSP synchronization, booking flow, and roaming settlement at protocol boundaries",
            extreme: "reconciling roaming authorization, tariff exchange, and multi-party settlement across current OCPI networks",
        },
        GeneratorFamily::McpA2aOps => Descriptor {
            title: "MCP and A2A ops",
            protocol: Some(ProtocolAdapter::Mcp),
            schema: Some(("agent-surface", "2026-04")),
            low: "routing tool calls and agent messages across MCP and A2A surfaces",
            high: "aligning MCP auth, remote tool execution, and AgentCard discovery across agent networks",
            extreme: "coordinating MCP resource exchange and A2A task handoff with explicit auth boundaries and policy controls",
        },
        GeneratorFamily::StreamingBusOps => Descriptor {
            title: "Streaming bus ops",
            protocol: Some(ProtocolAdapter::Kafka),
            schema: Some(("stream-bus-event", "2026-04")),
            low: "moving events through MQTT, NATS, and Kafka channels with replay-safe routing",
            high: "balancing consumer lag, retention, and JetStream-or-Kafka durability across streaming workloads",
            extreme: "coordinating brokered telemetry, replay windows, and stream processor contracts across multi-bus topologies",
        },
        GeneratorFamily::ServiceMeshRpcOps => Descriptor {
            title: "Service mesh RPC ops",
            protocol: Some(ProtocolAdapter::Grpc),
            schema: Some(("rpc-contract", "2026-04")),
            low: "serving typed RPC and federated API traffic across internal service boundaries",
            high: "tuning gRPC, GraphQL federation, and timeout budgets across routed service meshes",
            extreme: "reconciling streaming RPC, schema composition, and retry propagation across mesh-aware service fabrics",
        },
    }
}

fn selected_message(descriptor: Descriptor, jargon_level: JargonLevel) -> &'static str {
    match jargon_level {
        JargonLevel::Low | JargonLevel::Medium => descriptor.low,
        JargonLevel::High => descriptor.high,
        JargonLevel::Extreme => descriptor.extreme,
    }
}

pub fn title_for_family(family: GeneratorFamily) -> &'static str {
    descriptor(family).title
}

pub fn protocol_for_family(family: GeneratorFamily) -> Option<ProtocolAdapter> {
    descriptor(family).protocol
}

pub fn schema_ref_for_family(family: GeneratorFamily) -> Option<SchemaRef> {
    descriptor(family).schema.map(|(name, version)| SchemaRef {
        name: name.to_string(),
        version: version.to_string(),
    })
}

pub fn build_event(
    family: GeneratorFamily,
    config: &SessionConfig,
    sequence: u64,
    flavors: &[ScenarioFlavor],
) -> EventEnvelope {
    let descriptor = descriptor(family);
    let mut context = BTreeMap::new();
    context.insert("family".to_string(), family.label().to_string());
    context.insert(
        "devType".to_string(),
        serde_json::to_string(&config.dev_type)
            .unwrap_or_else(|_| "\"unknown\"".to_string())
            .replace('"', ""),
    );
    context.insert("project".to_string(), config.project_name.clone());
    context.insert(
        "complexity".to_string(),
        config.complexity.activity_count().to_string(),
    );
    context.insert(
        "outputFormat".to_string(),
        format!("{:?}", config.output_format).to_lowercase(),
    );
    if !config.framework.is_empty() {
        context.insert("framework".to_string(), config.framework.clone());
    }
    if let Some(seed) = config.seed {
        context.insert("seed".to_string(), seed.to_string());
    }
    if let Some(protocol) = descriptor.protocol {
        context.insert(
            "protocolAdapter".to_string(),
            serde_json::to_string(&protocol)
                .unwrap_or_else(|_| "\"unknown\"".to_string())
                .replace('"', ""),
        );
    }

    EventEnvelope {
        event_type: "activity".to_string(),
        sequence,
        timestamp: format!("T+{:06}ms", sequence * 137),
        message: selected_message(descriptor, config.jargon_level).to_string(),
        family: Some(family),
        protocol: descriptor.protocol,
        schema_ref: schema_ref_for_family(family),
        flavors: flavors.to_vec(),
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
