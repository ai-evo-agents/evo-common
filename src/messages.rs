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
    #[serde(default)]
    pub parent_id: Option<String>,
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
    #[serde(default)]
    pub parent_id: Option<String>,
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
    #[serde(default)]
    pub parent_id: String,
    pub created_at: String,
    pub updated_at: String,
}

fn default_task_limit() -> u32 {
    50
}

fn default_memory_limit() -> u32 {
    20
}

fn default_empty_object() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

// ─── Memory system types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScope {
    System,
    Agent,
    Pipeline,
    Skill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Case,
    Pattern,
    Fact,
    Preference,
    Resource,
    Event,
}

/// A single tier entry (l0/l1/l2) for memory creation/update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTierEntry {
    pub tier: String,
    pub content: String,
}

/// Agent stores a memory into king.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStore {
    pub scope: MemoryScope,
    pub category: MemoryCategory,
    #[serde(default)]
    pub key: String,
    #[serde(default = "default_empty_object")]
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub skill_id: String,
    #[serde(default)]
    pub relevance_score: f64,
    #[serde(default)]
    pub tiers: Vec<MemoryTierEntry>,
    #[serde(default)]
    pub task_id: Option<String>,
}

/// Agent queries memories from king.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub query: String,
    #[serde(default)]
    pub scope: Option<MemoryScope>,
    #[serde(default)]
    pub category: Option<MemoryCategory>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default = "default_memory_limit")]
    pub limit: u32,
}

/// A single tier in a returned memory record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTierRecord {
    pub id: String,
    pub memory_id: String,
    pub tier: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Serialized memory record returned in results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub scope: String,
    pub category: String,
    pub key: String,
    #[serde(default)]
    pub tiers: Vec<MemoryTierRecord>,
    #[serde(default = "default_empty_object")]
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub skill_id: String,
    #[serde(default)]
    pub relevance_score: f64,
    #[serde(default)]
    pub access_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// King returns matching memories to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResult {
    pub memories: Vec<MemoryRecord>,
    pub count: u32,
}

/// Broadcast when a memory is created, updated, or deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryChanged {
    pub action: String,
    #[serde(default)]
    pub memory: Option<MemoryRecord>,
    #[serde(default)]
    pub memory_id: Option<String>,
}

// ─── Task Room messages ─────────────────────────────────────────────────────

/// King invites agents to join a task room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInvite {
    pub task_id: String,
    pub task_type: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// King streams output data into a task room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    pub task_id: String,
    pub request_id: String,
    /// Source of output: `"pty"` or `"llm"`.
    pub source: String,
    pub delta: String,
    pub chunk_index: u32,
    #[serde(default)]
    pub is_final: bool,
}

/// King requests evaluation of a completed task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvaluate {
    pub task_id: String,
    pub task_type: String,
    /// Accumulated output text (truncated if very large).
    #[serde(default)]
    pub output_summary: String,
    #[serde(default)]
    pub exit_code: Option<i32>,
    #[serde(default)]
    pub latency_ms: Option<u64>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Evaluation agent reports a task summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSummary {
    pub task_id: String,
    pub agent_id: String,
    pub summary: String,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub evaluation: serde_json::Value,
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

    // Debug events
    pub const DEBUG_PROMPT: &str = "debug:prompt";
    pub const DEBUG_RESPONSE: &str = "debug:response";
    pub const DEBUG_STREAM: &str = "debug:stream";

    // Memory events
    pub const MEMORY_STORE: &str = "memory:store";
    pub const MEMORY_QUERY: &str = "memory:query";
    pub const MEMORY_UPDATE: &str = "memory:update";
    pub const MEMORY_DELETE: &str = "memory:delete";
    pub const MEMORY_CHANGED: &str = "memory:changed";

    // Task Room events
    pub const TASK_INVITE: &str = "task:invite";
    pub const TASK_JOIN: &str = "task:join";
    pub const TASK_OUTPUT: &str = "task:output";
    pub const TASK_EVALUATE: &str = "task:evaluate";
    pub const TASK_SUMMARY: &str = "task:summary";
    pub const TASK_LOG: &str = "task:log";

    // Rooms
    pub const ROOM_KERNEL: &str = "kernel";
    pub const ROOM_ROLE_PREFIX: &str = "role:";
    pub const ROOM_TASK_PREFIX: &str = "task:";
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
            parent_id: None,
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

    #[test]
    fn deserialize_task_create_with_parent_id() {
        let msg: TaskCreate =
            serde_json::from_str(r#"{"task_type": "subtask", "parent_id": "abc-123"}"#).unwrap();
        assert_eq!(msg.parent_id, Some("abc-123".to_string()));
    }

    #[test]
    fn deserialize_task_create_without_parent_id() {
        let msg: TaskCreate = serde_json::from_str(r#"{"task_type": "test"}"#).unwrap();
        assert!(msg.parent_id.is_none());
    }

    #[test]
    fn deserialize_task_list_with_parent_id() {
        let msg: TaskList = serde_json::from_str(r#"{"parent_id": "parent-001"}"#).unwrap();
        assert_eq!(msg.parent_id, Some("parent-001".to_string()));
        assert_eq!(msg.limit, 50);
    }

    #[test]
    fn serialize_memory_scope() {
        let scope = MemoryScope::Agent;
        let json = serde_json::to_string(&scope).unwrap();
        assert_eq!(json, r#""agent""#);
        let de: MemoryScope = serde_json::from_str(&json).unwrap();
        assert_eq!(de, MemoryScope::Agent);
    }

    #[test]
    fn serialize_memory_category() {
        let cat = MemoryCategory::Pattern;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""pattern""#);
        let de: MemoryCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(de, MemoryCategory::Pattern);
    }

    #[test]
    fn serialize_memory_store() {
        let msg = MemoryStore {
            scope: MemoryScope::Agent,
            category: MemoryCategory::Pattern,
            key: "memory://agent/learning/api_pattern".into(),
            metadata: serde_json::json!({"source": "pipeline"}),
            tags: vec!["discovery".into(), "api".into()],
            agent_id: "learning-001".into(),
            run_id: "".into(),
            skill_id: "".into(),
            relevance_score: 0.85,
            tiers: vec![
                MemoryTierEntry {
                    tier: "l0".into(),
                    content: "API discovery pattern".into(),
                },
                MemoryTierEntry {
                    tier: "l2".into(),
                    content: "Full detailed content...".into(),
                },
            ],
            task_id: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let de: MemoryStore = serde_json::from_str(&json).unwrap();
        assert_eq!(de.scope, MemoryScope::Agent);
        assert_eq!(de.category, MemoryCategory::Pattern);
        assert_eq!(de.tiers.len(), 2);
    }

    #[test]
    fn deserialize_memory_query_defaults() {
        let msg: MemoryQuery = serde_json::from_str(r#"{"query": "api discovery"}"#).unwrap();
        assert_eq!(msg.limit, 20);
        assert!(msg.scope.is_none());
        assert!(msg.task_id.is_none());
    }

    #[test]
    fn serialize_memory_changed() {
        let msg = MemoryChanged {
            action: "created".into(),
            memory: None,
            memory_id: Some("mem-001".into()),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let de: MemoryChanged = serde_json::from_str(&json).unwrap();
        assert_eq!(de.action, "created");
        assert_eq!(de.memory_id.unwrap(), "mem-001");
    }
}
