use std::process::Stdio;

mod common;
use common::{TempRepo, godharness};

fn temp_repo(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-stats", name)
}

fn temp_home(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-stats-home", name)
}

struct HookCall<'a> {
    repo: &'a std::path::Path,
    home: &'a std::path::Path,
    prompt: &'a str,
    session_id: &'a str,
    transcript_path: Option<&'a std::path::Path>,
}

#[allow(clippy::expect_used)]
fn run_adapter_hook_with_transcript(call: HookCall) {
    let mut command = godharness();
    command
        .arg("adapter-hook")
        .arg("claude-code")
        .arg("--event")
        .arg("user-prompt-submit")
        .current_dir(call.repo)
        .env("HOME", call.home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().expect("spawn adapter-hook");

    let prompt = call.prompt;
    let session_id = call.session_id;
    let stdin_json = match call.transcript_path {
        Some(path) => format!(
            r#"{{"prompt":"{prompt}","session_id":"{session_id}","transcript_path":"{}"}}"#,
            path.display()
        ),
        None => format!(r#"{{"prompt":"{prompt}","session_id":"{session_id}"}}"#),
    };

    use std::io::Write;
    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(stdin_json.as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait for adapter-hook");
    assert!(output.status.success());
}

fn run_adapter_hook(repo: &std::path::Path, home: &std::path::Path, prompt: &str) {
    run_adapter_hook_with_transcript(HookCall {
        repo,
        home,
        prompt,
        session_id: "stats-test",
        transcript_path: None,
    });
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
    let entries = parsed["entries"]
        .as_array()
        .expect("entries should be an array");

    let secrets_entry = entries
        .iter()
        .find(|entry| entry["id"] == "secrets-and-security")
        .expect("secrets-and-security should have fired");
    assert_eq!(secrets_entry["fires"], 2);
    assert!(secrets_entry["total_approx_tokens"].as_u64().unwrap() > 0);

    let cost = &parsed["cost"];
    assert!(cost["priced"].is_array());
    assert!(cost["unpriced_tokens"].is_u64());
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
    let before_parsed: serde_json::Value = serde_json::from_str(&before).expect("valid JSON");
    assert!(!before_parsed["entries"].as_array().unwrap().is_empty());

    let reset_output = run_stats(&repo.path, &home.path, &["--reset"]);
    assert!(reset_output.contains("cleared"));

    let after = run_stats(&repo.path, &home.path, &["--json"]);
    let after_parsed: serde_json::Value = serde_json::from_str(&after).expect("valid JSON");
    assert!(after_parsed["entries"].as_array().unwrap().is_empty());
}

#[allow(clippy::expect_used)]
#[test]
fn stats_prices_usage_by_the_model_detected_from_the_transcript() {
    let repo = temp_repo("model-detect");
    let home = temp_home("model-detect");
    godharness()
        .arg("init")
        .current_dir(&repo.path)
        .output()
        .expect("run godharness init");

    let transcript_path = repo.path.join("transcript.jsonl");
    std::fs::write(
        &transcript_path,
        r#"{"type":"user","message":{"role":"user"}}
{"type":"assistant","message":{"model":"claude-sonnet-4-6"}}
"#,
    )
    .expect("write fake transcript");

    run_adapter_hook_with_transcript(HookCall {
        repo: &repo.path,
        home: &home.path,
        prompt: "add a credential to config",
        session_id: "model-detect-session",
        transcript_path: Some(&transcript_path),
    });

    let stdout = run_stats(&repo.path, &home.path, &["--json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let priced = parsed["cost"]["priced"]
        .as_array()
        .expect("priced should be an array");

    let sonnet_entry = priced
        .iter()
        .find(|entry| entry["model"] == "claude-sonnet-4-6")
        .expect("claude-sonnet-4-6 should have been detected and priced");
    assert!(sonnet_entry["estimated_usd"].as_f64().unwrap() > 0.0);
    assert_eq!(parsed["cost"]["unpriced_tokens"], 0);
}

#[allow(clippy::expect_used)]
#[test]
fn stats_model_override_reprices_every_token_as_the_named_model() {
    let repo = temp_repo("model-override");
    let home = temp_home("model-override");
    godharness()
        .arg("init")
        .current_dir(&repo.path)
        .output()
        .expect("run godharness init");

    run_adapter_hook(&repo.path, &home.path, "add a credential to config");

    let stdout = run_stats(
        &repo.path,
        &home.path,
        &["--json", "--model", "claude-opus-4-6"],
    );
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let priced = parsed["cost"]["priced"]
        .as_array()
        .expect("priced should be an array");

    assert_eq!(priced.len(), 1);
    assert_eq!(priced[0]["model"], "claude-opus-4-6");
    assert_eq!(
        parsed["cost"]["unpriced_tokens"], 0,
        "the override should price events that had no detected model"
    );
}
