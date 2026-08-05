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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    struct TempRoot {
        path: std::path::PathBuf,
    }

    impl TempRoot {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "godharness-check-test-{name}-{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(path.join("docs/godharness")).expect("create temp docs dir");
            Self { path }
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn check_succeeds_with_no_repository_standards() {
        let root = TempRoot::new("empty");

        let report = run_check(&root.path).expect("check should succeed with just the suite");

        assert_eq!(report.standard_count, 9);
    }

    #[test]
    fn check_merges_a_valid_repository_standard() {
        let root = TempRoot::new("valid");
        fs::write(
            root.path.join("docs/godharness/custom.md"),
            "---\nid: custom\ntitle: Custom\n---\n\n## Rule\n\nDo the custom thing.\n",
        )
        .expect("write custom standard");

        let report =
            run_check(&root.path).expect("check should succeed with a valid custom standard");

        assert_eq!(report.standard_count, 10);
    }

    #[test]
    fn check_fails_on_an_invalid_repository_standard() {
        let root = TempRoot::new("invalid");
        fs::write(
            root.path.join("docs/godharness/broken.md"),
            "not a standard\n",
        )
        .expect("write broken standard");

        let result = run_check(&root.path);

        assert!(result.is_err());
    }

    #[test]
    fn check_fails_on_an_unknown_suite() {
        let root = TempRoot::new("unknown-suite");
        fs::write(
            root.path.join("godharness.yaml"),
            "version: 1\nsuites: [made-up@1]\n",
        )
        .expect("write config");

        let result = run_check(&root.path);

        assert!(result.is_err());
    }
}
