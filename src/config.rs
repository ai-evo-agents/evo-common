use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
    pub enabled: bool,
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
    fn parse_gateway_config() {
        let toml_str = r#"
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
"#;
        let config = GatewayConfig::from_toml(toml_str).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.providers.len(), 2);
        assert_eq!(config.providers[0].name, "openai");
    }

    #[test]
    fn roundtrip_gateway_config() {
        let config = GatewayConfig {
            server: ServerConfig {
                host: "127.0.0.1".into(),
                port: 3000,
            },
            providers: vec![ProviderConfig {
                name: "test".into(),
                base_url: "http://localhost:11434".into(),
                api_key_env: "TEST_KEY".into(),
                enabled: true,
                rate_limit: None,
            }],
        };
        let toml_str = config.to_toml().unwrap();
        let parsed = GatewayConfig::from_toml(&toml_str).unwrap();
        assert_eq!(parsed.server.port, 3000);
    }
}
