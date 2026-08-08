use std::path::Path;

use godharness_core::update_repository;

#[allow(clippy::expect_used)]
fn temp_repo(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "godharness-core-test-update-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("create temp repo root");
    path
}

fn cleanup(path: &Path) {
    let _ = std::fs::remove_dir_all(path);
}

#[test]
fn a_repository_with_no_config_reports_nothing_to_update() {
    let root = temp_repo("no-config");

    let report = update_repository(&root).expect("update should not fail on a missing config");

    assert!(report.suites_updated.is_empty());
    assert!(report.adapters_resynced.is_empty());

    cleanup(&root);
}

#[test]
fn a_repository_pinned_to_the_current_suite_version_reports_no_suite_change() {
    let root = temp_repo("current-suite");
    std::fs::write(
        root.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\n",
    )
    .expect("write config");

    let report = update_repository(&root).expect("update");

    assert!(report.suites_updated.is_empty());

    cleanup(&root);
}

#[test]
fn an_enabled_adapter_is_resynced() {
    let root = temp_repo("resync-adapter");
    std::fs::write(
        root.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nadapters:\n  claude-code: true\n",
    )
    .expect("write config");

    let report = update_repository(&root).expect("update");

    assert_eq!(report.adapters_resynced, vec!["claude-code".to_string()]);
    assert!(root.join(".claude/settings.json").exists());

    cleanup(&root);
}

#[test]
fn a_disabled_adapter_is_not_resynced() {
    let root = temp_repo("skip-disabled-adapter");
    std::fs::write(
        root.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nadapters:\n  claude-code: false\n",
    )
    .expect("write config");

    let report = update_repository(&root).expect("update");

    assert!(report.adapters_resynced.is_empty());
    assert!(!root.join(".claude/settings.json").exists());

    cleanup(&root);
}

#[test]
fn update_installs_skills_for_an_enabled_adapter() {
    let root = temp_repo("update-installs-skills");
    std::fs::write(
        root.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nadapters:\n  claude-code: true\n",
    )
    .expect("write config");

    update_repository(&root).expect("update");

    assert!(root.join(".claude/skills/atomic-commits/SKILL.md").exists());

    cleanup(&root);
}
