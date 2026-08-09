use std::collections::BTreeMap;

use serde::Deserialize;

pub mod adapters;
pub mod check;
pub mod doctor;
mod error;
pub mod graph;
pub mod init;
pub mod registry;
pub mod resolve;
pub mod skill;
pub mod standard;
pub mod suite;
pub mod update;

use error::string_error;

pub use adapters::{
    AdapterError, ClaudeCodeEvent, FieldMapping, HookRequest, InstallError, InstallReport,
    RenderedFile, SessionState, claude_code_hook_response, enable_adapter, render_shape_a,
    write_rendered_files,
};
pub use check::{
    CheckError, CheckReport, load_config, load_repository_graph, load_suite_skills, run_check,
};
pub use doctor::{DoctorReport, run_doctor};
pub use graph::{EdgeKind, GraphError, StandardGraph, build_graph, content_hash};
pub use init::{InitError, InitReport, run_init};
pub use resolve::{
    ResolvedSkill, ResolvedStandard, resolve, resolve_by_keyword_only, resolve_skills,
    resolve_skills_by_keyword_only,
};
pub use skill::{Skill, SkillError, parse_skill};
pub use standard::{Standard, StandardError, keyword_matches, parse_standard, path_matches};
pub use suite::{recommended_v1, recommended_v1_skills};
pub use update::{UpdateError, UpdateReport, update_repository};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub suites: Vec<String>,
    #[serde(default)]
    pub standards: Vec<String>,
    #[serde(default)]
    pub adapters: BTreeMap<String, bool>,
    #[serde(default, rename = "reinject-after-prompts")]
    pub reinject_after_prompts: u32,
}

string_error!(ConfigError, "invalid godharness configuration: ");

pub fn parse_config(yaml: &str) -> Result<Config, ConfigError> {
    serde_yaml::from_str(yaml).map_err(|error| ConfigError(error.to_string()))
}
