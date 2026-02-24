# evo-common

Shared types, protocols, and utilities for the Evo self-evolution agent system.

This is a Rust library crate consumed by every component in the Evo system. It defines the Socket.IO message protocol, configuration structs, skill manifest types, and structured logging initialization.

---

## Part of the Evo System

| Repository | Role |
|---|---|
| **evo-common** (this) | Shared types, protocol definitions, config structs, logging |
| [evo-gateway](https://github.com/ai-evo-agents/evo-gateway) | API aggregator (port 8080) unifying OpenAI, Anthropic, and local LLMs |
| [evo-king](https://github.com/ai-evo-agents/evo-king) | Central orchestrator with Socket.IO server (port 3000), config lifecycle, Turso DB |
| [evo-agents](https://github.com/ai-evo-agents/evo-agents) | Runner binary (kernel agents in separate repos) |

---

## Architecture

```
                        +------------------+
                        |   evo-gateway    |
                        |  port 8080       |
                        |  OpenAI          |
                        |  Anthropic       |
                        |  Local LLMs      |
                        +--------+---------+
                                 |
                                 | HTTP
                                 v
+------------------+    Socket.IO (port 3000)    +------------------+
|   evo-agents     | <-------------------------> |    evo-king      |
|                  |                             |                  |
|  runner binary   |    agent:register           |  orchestrator    |
|  kernel agents   |    agent:status             |  Socket.IO srv   |
|  user agents     |    agent:skill_report       |  config mgmt     |
|                  |    agent:health             |  Turso local DB  |
|  roles:          |    king:command      -----> |                  |
|  - learning      |    king:config_update ----> |                  |
|  - building      |    pipeline:next     <----> |                  |
|  - pre_load      |                             |                  |
|  - evaluation    |                             |                  |
|  - skill_manage  |                             |                  |
+------------------+                             +------------------+

         All components depend on evo-common for shared types.

Evolution pipeline (continuous cycle):

  Learning --> Building --> Pre-load --> Evaluation --> Skill Manage
     ^                                                       |
     +-------------------------------------------------------+
```

Communication between evo-king (server, using `socketioxide`) and evo-agents runners (clients, using `rust_socketio`) uses the Socket.IO event types defined in this crate.

---

## Modules

### `messages` - Socket.IO Event Types

All communication between king and runners is typed through structs and enums in this module.

#### Message Structs

```rust
// Runner announces itself to king on connect
pub struct AgentRegister {
    pub agent_id: String,
    pub role: AgentRole,
    pub capabilities: Vec<String>,
}

// Periodic heartbeat from runner to king
pub struct AgentStatus {
    pub agent_id: String,
    pub status: RunnerStatus,
    pub metrics: HashMap<String, serde_json::Value>,
}

// Runner reports result of a skill execution
pub struct AgentSkillReport {
    pub agent_id: String,
    pub skill_id: String,
    pub result: SkillResult,
    pub score: Option<f64>,
}

// Runner reports API health check results
pub struct AgentHealth {
    pub agent_id: String,
    pub health_checks: Vec<HealthCheck>,
}

// King sends a command to a specific agent
pub struct KingCommand {
    pub command: String,
    pub target_agent: String,
    pub params: HashMap<String, serde_json::Value>,
}

// King notifies runners of a config change
pub struct KingConfigUpdate {
    pub config_type: String,
    pub new_config_hash: String,
}

// Advances the evolution pipeline to the next stage
pub struct PipelineNext {
    pub stage: PipelineStage,
    pub artifact_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

// Individual health check result within AgentHealth
pub struct HealthCheck {
    pub name: String,
    pub endpoint: String,
    pub healthy: bool,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}
```

Note: Runners may include additional fields in the registration payload beyond the struct definition. For example, the `skills` field (a JSON array of skill names) is passed as untyped JSON alongside the typed `AgentRegister` fields. King extracts and persists these extra fields when handling `agent:register` events.

#### Enums

```rust
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    SkillManage,
    Learning,
    PreLoad,
    Building,
    Evaluation,
    User(String),  // custom/user-defined role
}

#[serde(rename_all = "snake_case")]
pub enum RunnerStatus {
    Starting,
    Ready,
    Busy,
    Error,
    Shutting,
}

#[serde(rename_all = "snake_case")]
pub enum SkillResult {
    Success,
    Failure(String),  // error message
    Partial(String),  // partial result description
}

#[serde(rename_all = "snake_case")]
pub enum PipelineStage {
    Learning,
    Building,
    PreLoad,
    Evaluation,
    SkillManage,
}

#[serde(rename_all = "snake_case")]
pub enum PipelineRunStatus {
    Running,
    Completed,
    Failed,
    TimedOut,
}

// Agent reports completion of a pipeline stage back to king
pub struct PipelineStageResult {
    pub run_id: String,
    pub stage: PipelineStage,
    pub agent_id: String,
    pub status: PipelineRunStatus,
    pub artifact_id: String,
    pub output: serde_json::Value,
    pub error: Option<String>,
}
```

#### Event Name Constants

The `messages::events` submodule provides string constants for all Socket.IO event names:

```rust
pub mod events {
    pub const AGENT_REGISTER: &str    = "agent:register";
    pub const AGENT_STATUS: &str      = "agent:status";
    pub const AGENT_SKILL_REPORT: &str = "agent:skill_report";
    pub const AGENT_HEALTH: &str      = "agent:health";
    pub const KING_COMMAND: &str      = "king:command";
    pub const KING_CONFIG_UPDATE: &str = "king:config_update";
    pub const PIPELINE_NEXT: &str          = "pipeline:next";
    pub const PIPELINE_STAGE_RESULT: &str  = "pipeline:stage_result";

    // Rooms
    pub const ROOM_KERNEL: &str      = "kernel";
    pub const ROOM_ROLE_PREFIX: &str  = "role:";
}
```

---

### `config` - Shared Configuration Structs

Parsed from TOML files by evo-gateway and evo-agents.

```rust
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub providers: Vec<ProviderConfig>,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,   // name of environment variable holding the key
    pub enabled: bool,
    pub rate_limit: Option<RateLimitConfig>,
}

pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

pub struct AgentConfig {
    pub role: String,
    pub skills: Vec<String>,
    pub king_address: String,
}
```

`GatewayConfig` provides `from_toml(&str)` and `to_toml()` methods. `AgentConfig` provides `from_toml(&str)`.

---

### `skill` - Skill Manifest Types

Describes a skill's interface and runtime configuration. Loaded from TOML files by evo-agents.

```rust
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub inputs: Vec<SkillIO>,
    pub outputs: Vec<SkillIO>,
    pub dependencies: Vec<String>,  // other skill names
    pub has_code: bool,             // whether this skill ships executable code
}

pub struct SkillIO {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: Option<String>,
}

pub struct SkillConfig {
    pub endpoints: Vec<SkillEndpoint>,
    pub auth_ref: Option<String>,              // env var name for auth token
    pub extra: HashMap<String, serde_json::Value>,
}

pub struct SkillEndpoint {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
}

#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}
```

`SkillManifest` and `SkillConfig` each provide `from_toml(&str)`.

---

### `logging` - Structured Logging

Initializes `tracing` with dual output: JSON to a daily rolling log file and human-readable to stdout.

```rust
// Returns the log directory path.
// Source: EVO_LOG_DIR env var, default: ./logs
pub fn log_dir() -> PathBuf

// Sets up the global tracing subscriber. Must be called once at startup.
// Returns a WorkerGuard that must be held for the lifetime of the process
// to ensure buffered log lines are flushed before exit.
pub fn init_logging(component: &str) -> WorkerGuard
```

Log files are written to `{log_dir}/{component}.YYYY-MM-DD.log` in JSON format. Stdout output is plain text. The log level is controlled by the `RUST_LOG` environment variable (default: `info`).

---

## Socket.IO Protocol

| Event | Direction | Payload Type |
|---|---|---|
| `agent:register` | runner -> king | `AgentRegister` |
| `agent:status` | runner -> king | `AgentStatus` |
| `agent:skill_report` | runner -> king | `AgentSkillReport` |
| `agent:health` | runner -> king | `AgentHealth` |
| `king:command` | king -> runner | `KingCommand` |
| `king:config_update` | king -> runner | `KingConfigUpdate` |
| `pipeline:next` | king <-> runner | `PipelineNext` |
| `pipeline:stage_result` | runner -> king | `PipelineStageResult` |

All payloads are JSON-serialized using `serde_json`. Enum variants use `snake_case` serialization by default; `HttpMethod` uses `UPPERCASE`.

---

## Configuration Format Examples

### Gateway Configuration (`gateway.toml`)

```toml
[server]
host = "0.0.0.0"
port = 8080

[[providers]]
name = "openai"
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"
enabled = true

[[providers]]
name = "anthropic"
base_url = "https://api.anthropic.com/v1"
api_key_env = "ANTHROPIC_API_KEY"
enabled = true

[[providers]]
name = "ollama"
base_url = "http://localhost:11434/v1"
api_key_env = "OLLAMA_API_KEY"
enabled = true

[providers.rate_limit]
requests_per_minute = 60
burst_size = 10
```

### Agent Configuration (`agent.toml`)

```toml
role = "learning"
skills = ["web-search", "summarize"]
king_address = "http://localhost:3000"
```

### Skill Manifest (`manifest.toml`)

```toml
name = "web-search"
version = "0.1.0"
description = "Search the web for information"
capabilities = ["search", "summarize"]
has_code = false
dependencies = []

[[inputs]]
name = "query"
type = "string"
required = true
description = "Search query string"

[[outputs]]
name = "results"
type = "array"
required = true
description = "List of search result objects"
```

### Skill Config (`config.toml`)

```toml
auth_ref = "SEARCH_API_KEY"

[[endpoints]]
name = "search"
url = "https://api.search.com/v1/search"
method = "GET"

[endpoints.headers]
Accept = "application/json"
```

---

## Usage

Add `evo-common` as a dependency in `Cargo.toml`:

```toml
[dependencies]
evo-common = "0.2"
```

### Logging initialization

```rust
use evo_common::logging;

fn main() {
    // Hold the guard for the process lifetime to ensure log flushing
    let _guard = logging::init_logging("my-component");

    tracing::info!("Component started");
    // ...
}
```

### Sending a registration message

```rust
use evo_common::messages::{AgentRegister, AgentRole, events};
use serde_json;

let msg = AgentRegister {
    agent_id: "learning-001".to_string(),
    role: AgentRole::Learning,
    capabilities: vec!["discover".to_string(), "evaluate".to_string()],
};

let payload = serde_json::to_string(&msg)?;
socket.emit(events::AGENT_REGISTER, payload).await?;
```

### Loading a gateway config

```rust
use evo_common::config::GatewayConfig;
use std::fs;

let content = fs::read_to_string("gateway.toml")?;
let config = GatewayConfig::from_toml(&content)?;

println!("Listening on {}:{}", config.server.host, config.server.port);
```

### Parsing a skill manifest

```rust
use evo_common::skill::SkillManifest;
use std::fs;

let content = fs::read_to_string("manifest.toml")?;
let manifest = SkillManifest::from_toml(&content)?;

println!("Loaded skill: {} v{}", manifest.name, manifest.version);
```

---

## Build and Test

```sh
# Build the library
cargo build

# Run all unit tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Check without building
cargo check
```

---

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `serde` | 1.0 | Serialization/deserialization framework |
| `serde_json` | 1.0 | JSON encoding for Socket.IO payloads |
| `toml` | 0.8 | TOML config file parsing |
| `chrono` | 0.4 | Timestamps with serde support |
| `tracing` | 0.1 | Structured logging macros |
| `tracing-subscriber` | 0.3 | Tracing output (JSON + stdout, env-filter) |
| `tracing-appender` | 0.2 | Non-blocking rolling file appender |

---

## License

MIT
