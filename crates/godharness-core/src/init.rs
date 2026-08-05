use std::path::Path;

const STARTER_CONFIG: &str = "version: 1\nsuites: [recommended@1]\n";

const STARTER_STANDARD: &str = "\
---
id: example
title: Example Standard
keywords: [example]
paths: []
must-read: false
supersedes: []
relates-to: []
---

## Rule

Replace this with a real rule your team has actually agreed on.

## Why

A starter standard exists so docs/godharness/ isn't empty; delete or replace it before it
misleads anyone into thinking it's a real standard.

## How to apply

Delete this file once you've written a real standard, or edit it into one.
";

#[derive(Debug)]
pub struct InitError(String);

impl std::fmt::Display for InitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::error::Error for InitError {}

impl From<std::io::Error> for InitError {
    fn from(error: std::io::Error) -> Self {
        InitError(error.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct InitReport {
    pub config_created: bool,
    pub starter_standard_created: bool,
}

fn write_if_absent(path: &Path, contents: &str) -> Result<bool, InitError> {
    if path.exists() {
        return Ok(false);
    }
    std::fs::write(path, contents)?;
    Ok(true)
}

pub fn run_init(root: &Path) -> Result<InitReport, InitError> {
    let config_created = write_if_absent(&root.join("godharness.yaml"), STARTER_CONFIG)?;

    let standards_dir = root.join("docs/godharness");
    std::fs::create_dir_all(&standards_dir)?;
    let starter_standard_created =
        write_if_absent(&standards_dir.join("example.md"), STARTER_STANDARD)?;

    Ok(InitReport {
        config_created,
        starter_standard_created,
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
        let contents =
            fs::read_to_string(root.path.join("godharness.yaml")).expect("read config back");
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
}
