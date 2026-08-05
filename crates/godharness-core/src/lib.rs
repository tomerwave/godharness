use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub suites: Vec<String>,
    #[serde(default)]
    pub standards: Vec<String>,
    #[serde(default)]
    pub adapters: BTreeMap<String, bool>,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid godharness configuration: {}", self.0)
    }
}

impl std::error::Error for ConfigError {}

pub fn parse_config(yaml: &str) -> Result<Config, ConfigError> {
    serde_yaml::from_str(yaml).map_err(|error| ConfigError(error.to_string()))
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ResolvedContext {
    pub standards: Vec<String>,
}

pub fn resolve(_config: &Config, _prompt: Option<&str>, _paths: &[String]) -> ResolvedContext {
    ResolvedContext::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_config() {
        let yaml = "version: 1\nsuites: [recommended@1]\n";

        let config = parse_config(yaml).expect("valid config should parse");

        assert_eq!(config.version, 1);
        assert_eq!(config.suites, vec!["recommended@1".to_string()]);
        assert!(config.standards.is_empty());
        assert!(config.adapters.is_empty());
    }

    #[test]
    fn parses_full_config() {
        let yaml = "\
version: 1
suites: [recommended@1]
standards:
  - docs/engineering/**
  - docs/architecture/**
adapters:
  codex: true
  claude-code: true
  pi: false
";

        let config = parse_config(yaml).expect("valid config should parse");

        assert_eq!(
            config.standards,
            vec![
                "docs/engineering/**".to_string(),
                "docs/architecture/**".to_string(),
            ]
        );
        assert_eq!(config.adapters.get("codex"), Some(&true));
        assert_eq!(config.adapters.get("pi"), Some(&false));
    }

    #[test]
    fn rejects_config_missing_version() {
        let yaml = "suites: [recommended@1]\n";

        let result = parse_config(yaml);

        assert!(result.is_err());
    }

    #[test]
    fn resolve_returns_empty_result_for_now() {
        let config = parse_config("version: 1\n").expect("valid config should parse");

        let resolved = resolve(
            &config,
            Some("add a new endpoint"),
            &["src/main.rs".to_string()],
        );

        assert_eq!(resolved, ResolvedContext::default());
    }
}
