use std::process::Stdio;

mod common;
use common::{TempRepo, godharness};

fn temp_repo(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-stats", name)
}

fn temp_home(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-stats-home", name)
}

#[allow(clippy::expect_used)]
fn run_adapter_hook(repo: &std::path::Path, home: &std::path::Path, prompt: &str) {
    let mut child = godharness()
        .arg("adapter-hook")
        .arg("claude-code")
        .arg("--event")
        .arg("user-prompt-submit")
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn adapter-hook");

    use std::io::Write;
    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(format!(r#"{{"prompt":"{prompt}","session_id":"stats-test"}}"#).as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait for adapter-hook");
    assert!(output.status.success());
}

#[allow(clippy::expect_used)]
fn run_stats(repo: &std::path::Path, home: &std::path::Path, extra_args: &[&str]) -> String {
    let mut command = godharness();
    command
        .arg("stats")
        .args(extra_args)
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let output = command.output().expect("run godharness stats");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout should be utf8")
}

#[test]
fn stats_reports_no_usage_before_any_hook_has_fired() {
    let repo = temp_repo("empty");
    let home = temp_home("empty");
    godharness()
        .arg("init")
        .current_dir(&repo.path)
        .output()
        .expect("run godharness init");

    let stdout = run_stats(&repo.path, &home.path, &[]);

    assert!(stdout.contains("no recorded usage"));
}

#[test]
fn stats_records_and_aggregates_a_matched_standard() {
    let repo = temp_repo("records");
    let home = temp_home("records");
    godharness()
        .arg("init")
        .current_dir(&repo.path)
        .output()
        .expect("run godharness init");

    run_adapter_hook(&repo.path, &home.path, "add a credential to config");
    run_adapter_hook(&repo.path, &home.path, "add another credential");

    let stdout = run_stats(&repo.path, &home.path, &["--json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("value should be an array");

    let secrets_entry = entries
        .iter()
        .find(|entry| entry["id"] == "secrets-and-security")
        .expect("secrets-and-security should have fired");
    assert_eq!(secrets_entry["fires"], 2);
    assert!(secrets_entry["total_approx_tokens"].as_u64().unwrap() > 0);
}

#[test]
fn stats_reset_clears_the_usage_log() {
    let repo = temp_repo("reset");
    let home = temp_home("reset");
    godharness()
        .arg("init")
        .current_dir(&repo.path)
        .output()
        .expect("run godharness init");

    run_adapter_hook(&repo.path, &home.path, "add a credential to config");
    let before = run_stats(&repo.path, &home.path, &["--json"]);
    assert_ne!(before.trim(), "[]");

    let reset_output = run_stats(&repo.path, &home.path, &["--reset"]);
    assert!(reset_output.contains("cleared"));

    let after = run_stats(&repo.path, &home.path, &["--json"]);
    assert_eq!(after.trim(), "[]");
}
