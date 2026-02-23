use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Which wire protocol the provider speaks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    /// OpenAI-compatible REST API (OpenAI, OpenRouter, Ollama, vLLM, etc.)
    #[default]
    OpenAiCompatible,
    /// Anthropic Messages API — different auth headers and request format.
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    /// One or more env-var names whose values are API tokens.
    /// Multiple tokens enable a round-robin pool: ["KEY_1", "KEY_2", ...].
    /// Leave empty for unauthenticated providers (e.g. local Ollama).
    #[serde(default)]
    pub api_key_envs: Vec<String>,
    pub enabled: bool,
    /// Wire protocol this provider uses.
    #[serde(default)]
    pub provider_type: ProviderType,
    /// Optional extra HTTP headers sent on every request (e.g. OpenRouter's
    /// `HTTP-Referer` and `X-Title`).
    #[serde(default)]
    pub extra_headers: HashMap<String, String>,
    #[serde(default)]
    pub rate_limit: Option<RateLimitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub role: String,
    pub skills: Vec<String>,
    pub king_address: String,
}

impl GatewayConfig {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn from_json(content: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl AgentConfig {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gateway_config_with_pool() {
        let toml_str = r#"
[server]
host = "0.0.0.0"
port = 8080

[[providers]]
name = "openai"
base_url = "https://api.openai.com/v1"
api_key_envs = ["OPENAI_API_KEY_1", "OPENAI_API_KEY_2"]
enabled = true
provider_type = "open_ai_compatible"

[[providers]]
name = "anthropic"
base_url = "https://api.anthropic.com/v1"
api_key_envs = ["ANTHROPIC_API_KEY"]
enabled = true
provider_type = "anthropic"

[[providers]]
name = "openrouter"
base_url = "https://openrouter.ai/api/v1"
api_key_envs = ["OPENROUTER_API_KEY"]
enabled = true
provider_type = "open_ai_compatible"

[providers.extra_headers]
"HTTP-Referer" = "https://github.com/ai-evo-agents"
"X-Title" = "evo-gateway"
"#;
        let config = GatewayConfig::from_toml(toml_str).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.providers.len(), 3);
        assert_eq!(config.providers[0].api_key_envs.len(), 2);
        assert_eq!(config.providers[1].provider_type, ProviderType::Anthropic);
        assert!(config.providers[2].extra_headers.contains_key("HTTP-Referer"));
    }

    #[test]
    fn roundtrip_gateway_config_toml() {
        let config = GatewayConfig {
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 3000,
            },
            providers: vec![ProviderConfig {
                name: "test".into(),
                base_url: "http://localhost:11434".into(),
                api_key_envs: vec![],
                enabled: true,
                provider_type: ProviderType::OpenAiCompatible,
                extra_headers: HashMap::new(),
                rate_limit: None,
            }],
        };
        let toml_str = config.to_toml().unwrap();
        let parsed = GatewayConfig::from_toml(&toml_str).unwrap();
        assert_eq!(parsed.server.port, 3000);
        assert_eq!(parsed.providers[0].api_key_envs.len(), 0);
    }

    #[test]
    fn roundtrip_gateway_config_json() {
        let config = GatewayConfig {
            server: ServerConfig {
                host: "0.0.0.0".into(),
                port: 8080,
            },
            providers: vec![
                ProviderConfig {
                    name: "openai".into(),
                    base_url: "https://api.openai.com/v1".into(),
                    api_key_envs: vec!["OPENAI_API_KEY".into()],
                    enabled: true,
                    provider_type: ProviderType::OpenAiCompatible,
                    extra_headers: HashMap::new(),
                    rate_limit: None,
                },
                ProviderConfig {
                    name: "anthropic".into(),
                    base_url: "https://api.anthropic.com/v1".into(),
                    api_key_envs: vec!["ANTHROPIC_API_KEY".into()],
                    enabled: true,
                    provider_type: ProviderType::Anthropic,
                    extra_headers: HashMap::new(),
                    rate_limit: None,
                },
            ],
        };
        let json_str = config.to_json().unwrap();
        let parsed = GatewayConfig::from_json(&json_str).unwrap();
        assert_eq!(parsed.server.port, 8080);
        assert_eq!(parsed.providers.len(), 2);
        assert_eq!(parsed.providers[1].provider_type, ProviderType::Anthropic);
        assert_eq!(parsed.providers[0].api_key_envs[0], "OPENAI_API_KEY");
    }
}
