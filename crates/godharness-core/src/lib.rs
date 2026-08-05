use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

pub mod check;
pub mod doctor;
pub mod graph;
pub mod init;
pub mod resolve;
pub mod standard;
pub mod suite;

pub use check::{CheckError, CheckReport, load_repository_graph, run_check};
pub use doctor::{DoctorReport, run_doctor};
pub use graph::{EdgeKind, GraphError, StandardGraph, build_graph, content_hash};
pub use init::{InitError, InitReport, run_init};
pub use resolve::{ResolvedStandard, resolve};
pub use standard::{Standard, StandardError, keyword_matches, parse_standard, path_matches};
pub use suite::recommended_v1;

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
