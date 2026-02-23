use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegister {
    pub agent_id: String,
    pub role: AgentRole,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub agent_id: String,
    pub status: RunnerStatus,
    pub metrics: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkillReport {
    pub agent_id: String,
    pub skill_id: String,
    pub result: SkillResult,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHealth {
    pub agent_id: String,
    pub health_checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KingCommand {
    pub command: String,
    pub target_agent: String,
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KingConfigUpdate {
    pub config_type: String,
    pub new_config_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNext {
    pub stage: PipelineStage,
    pub artifact_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    SkillManage,
    Learning,
    PreLoad,
    Building,
    Evaluation,
    User(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunnerStatus {
    Starting,
    Ready,
    Busy,
    Error,
    Shutting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillResult {
    Success,
    Failure(String),
    Partial(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub endpoint: String,
    pub healthy: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    Learning,
    Building,
    PreLoad,
    Evaluation,
    SkillManage,
}

pub mod events {
    pub const AGENT_REGISTER: &str = "agent:register";
    pub const AGENT_STATUS: &str = "agent:status";
    pub const AGENT_SKILL_REPORT: &str = "agent:skill_report";
    pub const AGENT_HEALTH: &str = "agent:health";
    pub const KING_COMMAND: &str = "king:command";
    pub const KING_CONFIG_UPDATE: &str = "king:config_update";
    pub const PIPELINE_NEXT: &str = "pipeline:next";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_agent_register() {
        let msg = AgentRegister {
            agent_id: "learning-001".into(),
            role: AgentRole::Learning,
            capabilities: vec!["discover".into(), "evaluate".into()],
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: AgentRegister = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.agent_id, "learning-001");
        assert_eq!(deserialized.role, AgentRole::Learning);
    }

    #[test]
    fn serialize_pipeline_next() {
        let msg = PipelineNext {
            stage: PipelineStage::Building,
            artifact_id: "skill-xyz".into(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: PipelineNext = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.stage, PipelineStage::Building);
    }
}
