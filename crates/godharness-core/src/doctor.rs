use std::path::Path;

use crate::check::{CheckError, load_config, load_repository_graph};

#[derive(Debug, PartialEq)]
pub struct DoctorReport {
    pub standard_count: usize,
    pub enabled_adapters: Vec<String>,
}

pub fn run_doctor(root: &Path) -> Result<DoctorReport, CheckError> {
    let config = load_config(root)?;
    let graph = load_repository_graph(root)?;

    let mut enabled_adapters: Vec<String> = config
        .adapters
        .iter()
        .filter(|(_, enabled)| **enabled)
        .map(|(name, _)| name.clone())
        .collect();
    enabled_adapters.sort_unstable();

    Ok(DoctorReport {
        standard_count: graph.len(),
        enabled_adapters,
    })
}
