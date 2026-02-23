# evo-common

Shared types, protocols, and utilities used by all repos in the `ai-evo-agents` system.

## Quick Commands

```bash
# Build
cargo build

# Test (9 tests)
cargo test

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check
```

## Purpose

`evo-common` is a pure library crate (no binary). It defines the shared data model so all other crates compile against the same types. Any change here must be pushed and then dependency crates must run `cargo update evo-common` to pick up the new commit.

## Module Overview

| Module | Purpose |
|--------|---------|
| `src/messages.rs` | Socket.IO event payloads and agent/pipeline enums |
| `src/config.rs` | `GatewayConfig` with JSON serialization via `from_json` / `to_json` |
| `src/skill.rs` | `SkillManifest` and `SkillConfig` (parsed from skill `manifest.toml` / `config.toml`) |
| `src/logging.rs` | `init_logging(component)` — structured JSON logs to `$EVO_LOG_DIR/<component>.log` |

## Socket.IO Event Constants (`messages::events`)

| Constant | Direction | Description |
|----------|-----------|-------------|
| `agent:register` | runner → king | Runner announces itself on connect |
| `agent:status` | runner → king | Heartbeat every 30 s |
| `agent:skill_report` | runner → king | Skill evaluation result + score |
| `agent:health` | runner → king | API endpoint health check results |
| `king:command` | king → runner | Targeted command (e.g. `discover`) |
| `king:config_update` | king → broadcast | Gateway config changed, new hash |
| `pipeline:next` | king → runner | Advance to next pipeline stage |

## Key Types

```rust
// Agent identity
enum AgentRole { SkillManage, Learning, PreLoad, Building, Evaluation, User(String) }

// Pipeline stages (matches kernel agent roles)
enum PipelineStage { Learning, Building, PreLoad, Evaluation, SkillManage }

// Skill discovery/packaging artifact
struct AgentSkillReport { agent_id, skill_id, result: SkillResult, score: Option<f64> }

// Config lifecycle event
struct KingConfigUpdate { config_type, new_config_hash }
```

## Logging (`logging::init_logging`)

```rust
// Usage in any binary
let _log_guard = init_logging("gateway");  // component name becomes filename
```

- Log directory: `$EVO_LOG_DIR` (default `./logs/`)
- Rolling daily files: `logs/gateway.log`, `logs/king.log`, `logs/<role>.log`
- **File layer**: JSON format with target, thread IDs, file, line number
- **Stdout layer**: Human-readable with target
- Log level: `RUST_LOG` env var (default `info`)
- **Important**: keep `_log_guard` alive for the entire process duration (non-blocking appender flush)

## Dependency Update Flow

When you change a type in evo-common:

```bash
# 1. Push the change
git add . && git commit -m "feat: ..." && git push

# 2. In each dependent repo:
cargo update evo-common
cargo build
```

## Testing

```bash
cargo test
# Tests cover: config roundtrip JSON, message serialization, log dir resolution
```
