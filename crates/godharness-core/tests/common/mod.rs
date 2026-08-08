use std::path::{Path, PathBuf};

use godharness_core::Standard;

pub struct TempRoot {
    pub path: PathBuf,
}

impl TempRoot {
    #[allow(clippy::expect_used, dead_code)]
    pub fn new(prefix: &str, name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("{prefix}-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp root");
        Self { path }
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[allow(dead_code)]
pub fn standard(id: &str, keywords: &[&str], paths: &[&str], must_read: bool) -> Standard {
    Standard {
        id: id.to_string(),
        title: id.to_string(),
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        paths: paths.iter().map(|s| s.to_string()).collect(),
        must_read,
        supersedes: Vec::new(),
        relates_to: Vec::new(),
        rule: format!("Rule for {id}."),
        why: None,
        how_to_apply: None,
        source_path: Path::new(&format!("{id}.md")).to_path_buf(),
    }
}
