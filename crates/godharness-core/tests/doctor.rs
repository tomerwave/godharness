use std::fs;

use godharness_core::run_doctor;

struct TempRoot {
    path: std::path::PathBuf,
}

impl TempRoot {
    #[allow(clippy::expect_used)]
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

    assert_eq!(report.standard_count, 29);
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
