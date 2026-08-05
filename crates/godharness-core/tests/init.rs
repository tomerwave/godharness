use std::fs;

use godharness_core::{InitReport, run_init};

struct TempRoot {
    path: std::path::PathBuf,
}

impl TempRoot {
    #[allow(clippy::expect_used)]
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "godharness-init-test-{name}-{}",
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
fn creates_config_and_starter_standard_on_a_fresh_repository() {
    let root = TempRoot::new("fresh");

    let report = run_init(&root.path).expect("init should succeed on a fresh repository");

    assert_eq!(
        report,
        InitReport {
            config_created: true,
            starter_standard_created: true
        }
    );
    assert!(root.path.join("godharness.yaml").exists());
    assert!(root.path.join("docs/godharness/example.md").exists());
}

#[test]
fn does_not_overwrite_an_existing_config() {
    let root = TempRoot::new("existing-config");
    fs::write(
        root.path.join("godharness.yaml"),
        "version: 1\nsuites: []\n",
    )
    .expect("write existing config");

    let report = run_init(&root.path).expect("init should succeed with an existing config");

    assert!(!report.config_created);
    let contents = fs::read_to_string(root.path.join("godharness.yaml")).expect("read config back");
    assert_eq!(contents, "version: 1\nsuites: []\n");
}

#[test]
fn running_twice_is_a_no_op_the_second_time() {
    let root = TempRoot::new("twice");

    run_init(&root.path).expect("first init should succeed");
    let second = run_init(&root.path).expect("second init should succeed");

    assert_eq!(
        second,
        InitReport {
            config_created: false,
            starter_standard_created: false
        }
    );
}
