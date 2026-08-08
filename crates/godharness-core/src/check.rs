use std::path::Path;

use crate::error::string_error;
use crate::graph::{GraphError, StandardGraph, build_graph};
use crate::skill::Skill;
use crate::standard::{Standard, StandardError, parse_standard};
use crate::suite::{recommended_v1, recommended_v1_skills};
use crate::{Config, ConfigError, parse_config};

const DEFAULT_STANDARDS_GLOB: &str = "docs/godharness/**/*.md";

string_error!(
    CheckError,
    "",
    from: ConfigError, StandardError, GraphError, std::io::Error, glob::PatternError, glob::GlobError,
);

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
        reinject_after_prompts: 0,
    }
}

pub fn load_config(root: &Path) -> Result<Config, CheckError> {
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

pub fn enabled_adapter_names(config: &Config) -> Vec<String> {
    let mut names: Vec<String> = config
        .adapters
        .iter()
        .filter(|(_, enabled)| **enabled)
        .map(|(name, _)| name.clone())
        .collect();
    names.sort_unstable();
    names
}

pub fn load_suite_skills(config: &Config) -> Vec<Skill> {
    config
        .suites
        .iter()
        .flat_map(|suite| match suite.as_str() {
            "recommended@1" => recommended_v1_skills(),
            _ => Vec::new(),
        })
        .collect()
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

pub fn load_repository_graph(root: &Path) -> Result<StandardGraph, CheckError> {
    let config = load_config(root)?;
    let mut standards = load_suite_standards(&config)?;
    standards.extend(load_repository_standards(root, &config)?);
    Ok(build_graph(standards)?)
}

pub fn run_check(root: &Path) -> Result<CheckReport, CheckError> {
    let graph = load_repository_graph(root)?;
    Ok(CheckReport {
        standard_count: graph.len(),
    })
}
