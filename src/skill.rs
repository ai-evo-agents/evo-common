use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub inputs: Vec<SkillIO>,
    pub outputs: Vec<SkillIO>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub has_code: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillIO {
    pub name: String,
    pub r#type: String,
    #[serde(default)]
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub endpoints: Vec<SkillEndpoint>,
    #[serde(default)]
    pub auth_ref: Option<String>,
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEndpoint {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl SkillManifest {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

impl SkillConfig {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skill_manifest() {
        let toml_str = r#"
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
description = "Search query"

[[outputs]]
name = "results"
type = "array"
required = true
description = "Search results"
"#;
        let manifest = SkillManifest::from_toml(toml_str).unwrap();
        assert_eq!(manifest.name, "web-search");
        assert_eq!(manifest.capabilities.len(), 2);
        assert!(manifest.inputs[0].required);
    }

    #[test]
    fn parse_skill_config() {
        let toml_str = r#"
auth_ref = "SEARCH_API_KEY"

[[endpoints]]
name = "search"
url = "https://api.search.com/v1/search"
method = "GET"

[endpoints.headers]
Accept = "application/json"
"#;
        let config = SkillConfig::from_toml(toml_str).unwrap();
        assert_eq!(config.endpoints[0].method, HttpMethod::Get);
        assert_eq!(config.auth_ref.unwrap(), "SEARCH_API_KEY");
    }
}
