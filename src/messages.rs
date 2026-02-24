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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PipelineRunStatus {
    Running,
    Completed,
    Failed,
    TimedOut,
}

/// Agent reports completion of a pipeline stage back to king.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStageResult {
    pub run_id: String,
    pub stage: PipelineStage,
    pub agent_id: String,
    pub status: PipelineRunStatus,
    pub artifact_id: String,
    pub output: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

// ─── Task management messages ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCreate {
    pub task_type: String,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default = "default_empty_object")]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUpdate {
    pub task_id: String,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGet {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
    #[serde(default = "default_task_limit")]
    pub limit: u32,
    #[serde(default)]
    pub status: Option<TaskStatus>,
    #[serde(default)]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDelete {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecord {
    pub id: String,
    pub task_type: String,
    pub status: String,
    pub agent_id: String,
    pub payload: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

fn default_task_limit() -> u32 {
    50
}

fn default_empty_object() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

pub mod events {
    pub const AGENT_REGISTER: &str = "agent:register";
    pub const AGENT_STATUS: &str = "agent:status";
    pub const AGENT_SKILL_REPORT: &str = "agent:skill_report";
    pub const AGENT_HEALTH: &str = "agent:health";
    pub const KING_COMMAND: &str = "king:command";
    pub const KING_CONFIG_UPDATE: &str = "king:config_update";
    pub const PIPELINE_NEXT: &str = "pipeline:next";

    // Task management events
    pub const TASK_CREATE: &str = "task:create";
    pub const TASK_UPDATE: &str = "task:update";
    pub const TASK_GET: &str = "task:get";
    pub const TASK_LIST: &str = "task:list";
    pub const TASK_DELETE: &str = "task:delete";
    pub const TASK_CHANGED: &str = "task:changed";

    // Pipeline coordination events
    pub const PIPELINE_STAGE_RESULT: &str = "pipeline:stage_result";

    // Rooms
    pub const ROOM_KERNEL: &str = "kernel";
    pub const ROOM_ROLE_PREFIX: &str = "role:";
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

    #[test]
    fn serialize_task_status() {
        let status = TaskStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""in_progress""#);
        let de: TaskStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, TaskStatus::InProgress);
    }

    #[test]
    fn serialize_task_create() {
        let msg = TaskCreate {
            task_type: "build".into(),
            agent_id: Some("building-001".into()),
            payload: serde_json::json!({"skill_id": "web-search"}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let de: TaskCreate = serde_json::from_str(&json).unwrap();
        assert_eq!(de.task_type, "build");
        assert_eq!(de.agent_id.unwrap(), "building-001");
    }

    #[test]
    fn deserialize_task_list_defaults() {
        let msg: TaskList = serde_json::from_str("{}").unwrap();
        assert_eq!(msg.limit, 50);
        assert!(msg.status.is_none());
        assert!(msg.agent_id.is_none());
    }

    #[test]
    fn serialize_pipeline_run_status() {
        let status = PipelineRunStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""running""#);
        let de: PipelineRunStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, PipelineRunStatus::Running);

        let timed_out = PipelineRunStatus::TimedOut;
        let json = serde_json::to_string(&timed_out).unwrap();
        assert_eq!(json, r#""timed_out""#);
    }

    #[test]
    fn serialize_pipeline_stage_result() {
        let result = PipelineStageResult {
            run_id: "run-001".into(),
            stage: PipelineStage::Learning,
            agent_id: "learning-001".into(),
            status: PipelineRunStatus::Completed,
            artifact_id: "artifact-xyz".into(),
            output: serde_json::json!({"candidates": 3}),
            error: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        let de: PipelineStageResult = serde_json::from_str(&json).unwrap();
        assert_eq!(de.run_id, "run-001");
        assert_eq!(de.stage, PipelineStage::Learning);
        assert_eq!(de.status, PipelineRunStatus::Completed);
        assert!(de.error.is_none());
    }

    #[test]
    fn serialize_pipeline_stage_result_with_error() {
        let result = PipelineStageResult {
            run_id: "run-002".into(),
            stage: PipelineStage::Building,
            agent_id: "building-001".into(),
            status: PipelineRunStatus::Failed,
            artifact_id: "".into(),
            output: serde_json::Value::Null,
            error: Some("build failed: missing dependency".into()),
        };
        let json = serde_json::to_string(&result).unwrap();
        let de: PipelineStageResult = serde_json::from_str(&json).unwrap();
        assert_eq!(de.status, PipelineRunStatus::Failed);
        assert_eq!(de.error.unwrap(), "build failed: missing dependency");
    }

    #[test]
    fn serialize_task_update_partial() {
        let msg = TaskUpdate {
            task_id: "abc-123".into(),
            status: Some(TaskStatus::Completed),
            agent_id: None,
            payload: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let de: TaskUpdate = serde_json::from_str(&json).unwrap();
        assert_eq!(de.task_id, "abc-123");
        assert_eq!(de.status, Some(TaskStatus::Completed));
        assert!(de.agent_id.is_none());
    }
}
