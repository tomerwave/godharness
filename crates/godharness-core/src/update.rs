use std::path::Path;

use crate::Config;
use crate::adapters::{InstallError, enable_adapter};
use crate::check::{CheckError, enabled_adapter_names, load_config};

const KNOWN_SUITES: &[&str] = &["recommended@1"];

#[derive(Debug)]
pub struct UpdateError(String);

impl std::fmt::Display for UpdateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::error::Error for UpdateError {}

impl From<CheckError> for UpdateError {
    fn from(error: CheckError) -> Self {
        UpdateError(error.to_string())
    }
}

impl From<InstallError> for UpdateError {
    fn from(error: InstallError) -> Self {
        UpdateError(error.to_string())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct UpdateReport {
    pub suites_updated: Vec<String>,
    pub adapters_resynced: Vec<String>,
}

fn suite_family(suite: &str) -> Option<&str> {
    suite.rsplit_once('@').map(|(family, _)| family)
}

fn latest_known_version(family: &str) -> Option<&'static str> {
    KNOWN_SUITES
        .iter()
        .filter(|known| suite_family(known) == Some(family))
        .max()
        .copied()
}

fn updated_suite(pinned: &str) -> Option<String> {
    let family = suite_family(pinned)?;
    let latest = latest_known_version(family)?;

    if latest == pinned {
        None
    } else {
        Some(latest.to_string())
    }
}

fn resync_adapters(root: &Path, config: &Config) -> Result<Vec<String>, UpdateError> {
    let mut resynced = Vec::new();

    for name in enabled_adapter_names(config) {
        enable_adapter(root, &name)?;
        resynced.push(name);
    }

    Ok(resynced)
}

fn updated_suites(config: &Config) -> Vec<String> {
    config
        .suites
        .iter()
        .filter_map(|suite| updated_suite(suite))
        .collect()
}

pub fn update_repository(root: &Path) -> Result<UpdateReport, UpdateError> {
    if !root.join("godharness.yaml").exists() {
        return Ok(UpdateReport::default());
    }

    let config = load_config(root)?;

    Ok(UpdateReport {
        suites_updated: updated_suites(&config),
        adapters_resynced: resync_adapters(root, &config)?,
    })
}
