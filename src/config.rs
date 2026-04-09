use crate::types::{Complexity, DevelopmentType, JargonLevel, OutputFormat};

pub struct SessionConfig {
    pub dev_type: DevelopmentType,
    pub jargon_level: JargonLevel,
    pub complexity: Complexity,
    pub alerts_enabled: bool,
    pub project_name: String,
    pub minimal_output: bool,
    pub team_activity: bool,
    pub framework: String,
    pub seed: Option<u64>,
    pub output_format: OutputFormat,
    pub no_color: bool,
    pub trace_enabled: bool,
}
