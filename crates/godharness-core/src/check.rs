use std::path::Path;

use crate::graph::{GraphError, build_graph};
use crate::standard::{Standard, StandardError, parse_standard};
use crate::suite::recommended_v1;
use crate::{Config, ConfigError, parse_config};

const DEFAULT_STANDARDS_GLOB: &str = "docs/godharness/**/*.md";

#[derive(Debug)]
pub struct CheckError(String);

impl std::fmt::Display for CheckError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::error::Error for CheckError {}

impl From<ConfigError> for CheckError {
    fn from(error: ConfigError) -> Self {
        CheckError(error.to_string())
    }
}

impl From<StandardError> for CheckError {
    fn from(error: StandardError) -> Self {
        CheckError(error.to_string())
    }
}

impl From<GraphError> for CheckError {
    fn from(error: GraphError) -> Self {
        CheckError(error.to_string())
    }
}

impl From<std::io::Error> for CheckError {
    fn from(error: std::io::Error) -> Self {
        CheckError(error.to_string())
    }
}

impl From<glob::PatternError> for CheckError {
    fn from(error: glob::PatternError) -> Self {
        CheckError(error.to_string())
    }
}

impl From<glob::GlobError> for CheckError {
    fn from(error: glob::GlobError) -> Self {
        CheckError(error.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct CheckReport {
    pub standard_count: usize,
}

fn default_config() -> Config {
    Config {
        version: 1,
        suites: vec!["recommended@1".to_string()],
        standards: vec![DEFAULT_STANDARDS_GLOB.to_string()],
        adapters: Default::default(),
    }
}

fn load_config(root: &Path) -> Result<Config, CheckError> {
    let config_path = root.join("godharness.yaml");
    if !config_path.exists() {
        return Ok(default_config());
    }
    Ok(parse_config(&std::fs::read_to_string(config_path)?)?)
}

fn load_suite_standards(config: &Config) -> Result<Vec<Standard>, CheckError> {
    let mut standards = Vec::new();
    for suite in &config.suites {
        match suite.as_str() {
            "recommended@1" => standards.extend(recommended_v1()?),
            other => return Err(CheckError(format!("unknown suite: {other}"))),
        }
    }
    Ok(standards)
}

fn standards_globs(config: &Config) -> Vec<String> {
    if config.standards.is_empty() {
        vec![DEFAULT_STANDARDS_GLOB.to_string()]
    } else {
        config.standards.clone()
    }
}

fn load_standard_at(path: &Path) -> Result<Standard, CheckError> {
    let document = std::fs::read_to_string(path)?;
    Ok(parse_standard(&document, path)?)
}

fn load_matching_standards(root: &Path, pattern: &str) -> Result<Vec<Standard>, CheckError> {
    let joined = root.join(pattern).to_string_lossy().into_owned();
    let mut standards = Vec::new();
    for entry in glob::glob(&joined)? {
        standards.push(load_standard_at(&entry?)?);
    }
    Ok(standards)
}

fn load_repository_standards(root: &Path, config: &Config) -> Result<Vec<Standard>, CheckError> {
    let mut standards = Vec::new();
    for pattern in standards_globs(config) {
        standards.extend(load_matching_standards(root, &pattern)?);
    }
    Ok(standards)
}

pub fn run_check(root: &Path) -> Result<CheckReport, CheckError> {
    let config = load_config(root)?;
    let mut standards = load_suite_standards(&config)?;
    standards.extend(load_repository_standards(root, &config)?);
    let graph = build_graph(standards)?;
    Ok(CheckReport {
        standard_count: graph.len(),
    })
}
