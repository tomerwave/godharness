use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

pub mod graph;
pub mod standard;

pub use graph::{EdgeKind, GraphError, StandardGraph, build_graph, content_hash};
pub use standard::{Standard, StandardError, keyword_matches, parse_standard, path_matches};

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
