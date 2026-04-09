use crate::config::SessionConfig;
use crate::domain::{EventEnvelope, GeneratorFamily, ScenarioFlavor};
use rand::rngs::StdRng;

use super::common;

pub fn render(
    config: &SessionConfig,
    _rng: &mut StdRng,
    sequence: u64,
    flavors: &[ScenarioFlavor],
) -> EventEnvelope {
    common::build_event(
        GeneratorFamily::ProofAndSequencerOps,
        config,
        sequence,
        flavors,
    )
}
