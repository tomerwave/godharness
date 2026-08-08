use std::fs;

use godharness_core::run_check;

struct TempRoot {
    path: std::path::PathBuf,
}

impl TempRoot {
    #[allow(clippy::expect_used)]
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

    assert_eq!(report.standard_count, 29);
}

#[test]
fn check_merges_a_valid_repository_standard() {
    let root = TempRoot::new("valid");
    fs::write(
        root.path.join("docs/godharness/custom.md"),
        "---\nid: custom\ntitle: Custom\n---\n\n## Rule\n\nDo the custom thing.\n",
    )
    .expect("write custom standard");

    let report = run_check(&root.path).expect("check should succeed with a valid custom standard");

    assert_eq!(report.standard_count, 30);
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
