use std::fs;

use godharness_core::run_doctor;

mod common;
use common::TempRoot;

fn temp_root(name: &str) -> TempRoot {
    TempRoot::new("godharness-doctor-test", name)
}

#[test]
fn reports_the_suite_count_with_no_config() {
    let root = temp_root("no-config");

    let report = run_doctor(&root.path).expect("doctor should succeed with no config");

    assert_eq!(report.standard_count, 31);
    assert!(report.enabled_adapters.is_empty());
}

#[test]
fn reports_enabled_adapters_sorted() {
    let root = temp_root("adapters");
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
