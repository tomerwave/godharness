use std::collections::BTreeMap;
use std::fs;

use godharness_core::{Config, load_suite_skills, run_check};

mod common;
use common::TempRoot;

#[allow(clippy::expect_used)]
fn temp_root(name: &str) -> TempRoot {
    let root = TempRoot::new("godharness-check-test", name);
    fs::create_dir_all(root.path.join("docs/godharness")).expect("create temp docs dir");
    root
}

#[test]
fn check_succeeds_with_no_repository_standards() {
    let root = temp_root("empty");

    let report = run_check(&root.path).expect("check should succeed with just the suite");

    assert_eq!(report.standard_count, 32);
}

#[test]
fn check_merges_a_valid_repository_standard() {
    let root = temp_root("valid");
    fs::write(
        root.path.join("docs/godharness/custom.md"),
        "---\nid: custom\ntitle: Custom\n---\n\n## Rule\n\nDo the custom thing.\n",
    )
    .expect("write custom standard");

    let report = run_check(&root.path).expect("check should succeed with a valid custom standard");

    assert_eq!(report.standard_count, 33);
}

#[test]
fn check_fails_on_an_invalid_repository_standard() {
    let root = temp_root("invalid");
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
    let root = temp_root("unknown-suite");
    fs::write(
        root.path.join("godharness.yaml"),
        "version: 1\nsuites: [made-up@1]\n",
    )
    .expect("write config");

    let result = run_check(&root.path);

    assert!(result.is_err());
}

#[test]
fn load_suite_skills_resolves_recommended_v1() {
    let config = Config {
        version: 1,
        suites: vec!["recommended@1".to_string()],
        standards: vec![],
        adapters: BTreeMap::new(),
        reinject_after_prompts: 0,
    };

    let skills = load_suite_skills(&config);

    assert_eq!(skills.len(), 13);
}

#[test]
fn load_suite_skills_is_empty_with_no_suites() {
    let config = Config {
        version: 1,
        suites: vec![],
        standards: vec![],
        adapters: BTreeMap::new(),
        reinject_after_prompts: 0,
    };

    let skills = load_suite_skills(&config);

    assert!(skills.is_empty());
}
