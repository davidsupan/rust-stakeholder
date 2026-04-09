use clap::ValueEnum;
use serde::Serialize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentType {
    Backend,
    Frontend,
    Fullstack,
    DataScience,
    DevOps,
    Blockchain,
    MachineLearning,
    SystemsProgramming,
    GameDevelopment,
    Security,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JargonLevel {
    Low,
    Medium,
    High,
    Extreme,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Complexity {
    Low,
    Medium,
    High,
    Extreme,
}

impl Complexity {
    pub fn activity_count(self) -> usize {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Extreme => 4,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguagePack {
    English,
    Chinese,
    Russian,
    Spanish,
    Arabic,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityPersona {
    BugBountyOperator,
    IncidentCommander,
    ReverseEngineer,
    ThreatHunter,
    SocAnalyst,
    DarkMarketWatcher,
    CtiBriefWriter,
}
