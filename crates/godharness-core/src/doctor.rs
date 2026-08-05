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
                "godharness-doctor-test-{name}-{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).expect("create temp root");
            Self { path }
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn reports_the_suite_count_with_no_config() {
        let root = TempRoot::new("no-config");

        let report = run_doctor(&root.path).expect("doctor should succeed with no config");

        assert_eq!(report.standard_count, 9);
        assert!(report.enabled_adapters.is_empty());
    }

    #[test]
    fn reports_enabled_adapters_sorted() {
        let root = TempRoot::new("adapters");
        fs::write(
            root.path.join("godharness.yaml"),
            "version: 1\nsuites: [recommended@1]\nadapters:\n  pi: true\n  codex: true\n  claude-code: false\n",
        )
        .expect("write config");

        let report = run_doctor(&root.path).expect("doctor should succeed");

        assert_eq!(
            report.enabled_adapters,
            vec!["codex".to_string(), "pi".to_string()]
        );
    }
}
