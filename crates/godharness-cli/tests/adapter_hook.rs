use std::io::Write;
use std::process::Stdio;

mod common;
use common::{TempRepo, godharness};

#[allow(clippy::expect_used)]
fn run_adapter_hook_for(
    tool: &str,
    event: &str,
    stdin_json: &str,
    cwd: Option<&std::path::Path>,
) -> String {
    let mut command = godharness();
    command
        .arg("adapter-hook")
        .arg(tool)
        .arg("--event")
        .arg(event)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    let mut child = command.spawn().expect("spawn godharness adapter-hook");
    child
        .stdin
        .take()
        .expect("child stdin should be piped")
        .write_all(stdin_json.as_bytes())
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait for adapter-hook");
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout should be utf8")
}

fn run_adapter_hook(event: &str, stdin_json: &str, cwd: Option<&std::path::Path>) -> String {
    run_adapter_hook_for("claude-code", event, stdin_json, cwd)
}

#[allow(clippy::expect_used)]
fn assert_hook_response_contains(stdout: &str, event_name: &str, standard_id: &str) {
    let parsed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout should be valid JSON");
    assert_eq!(parsed["hookSpecificOutput"]["hookEventName"], event_name);
    let context = parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string");
    assert!(context.contains(standard_id));
}

struct SessionState {
    session_id: String,
}

impl SessionState {
    fn new(name: &str) -> Self {
        Self {
            session_id: format!("godharness-cli-test-{name}-{}", std::process::id()),
        }
    }
}

impl Drop for SessionState {
    fn drop(&mut self) {
        let path = std::env::temp_dir()
            .join("godharness")
            .join("sessions")
            .join(format!("{}.json", self.session_id));
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn user_prompt_submit_prints_matching_standards_as_real_hook_json() {
    let session = SessionState::new("prompt-match");
    let stdin_json = format!(
        r#"{{"prompt":"add a credential to config","session_id":"{}"}}"#,
        session.session_id
    );

    let stdout = run_adapter_hook("user-prompt-submit", &stdin_json, None);

    assert_hook_response_contains(&stdout, "UserPromptSubmit", "secrets-and-security");
}

#[test]
fn user_prompt_submit_prints_nothing_on_no_match() {
    let session = SessionState::new("no-match");
    let stdin_json = format!(
        r#"{{"prompt":"totally unrelated wording","session_id":"{}"}}"#,
        session.session_id
    );

    let stdout = run_adapter_hook("user-prompt-submit", &stdin_json, None);

    assert!(stdout.trim().is_empty());
}

#[test]
fn session_start_prints_must_read_standards_as_real_hook_json() {
    let session = SessionState::new("session-start");
    let stdin_json = format!(r#"{{"session_id":"{}"}}"#, session.session_id);

    let stdout = run_adapter_hook("session-start", &stdin_json, None);

    assert_hook_response_contains(&stdout, "SessionStart", "secrets-and-security");
}

#[test]
fn codex_user_prompt_submit_prints_matching_standards_as_real_hook_json() {
    let session = SessionState::new("codex-prompt-match");
    let stdin_json = format!(
        r#"{{"prompt":"add a credential to config","session_id":"{}"}}"#,
        session.session_id
    );

    let stdout = run_adapter_hook_for("codex", "user-prompt-submit", &stdin_json, None);

    assert_hook_response_contains(&stdout, "UserPromptSubmit", "secrets-and-security");
}

#[test]
fn codex_session_start_prints_must_read_standards_as_real_hook_json() {
    let session = SessionState::new("codex-session-start");
    let stdin_json = format!(r#"{{"session_id":"{}"}}"#, session.session_id);

    let stdout = run_adapter_hook_for("codex", "session-start", &stdin_json, None);

    assert_hook_response_contains(&stdout, "SessionStart", "secrets-and-security");
}

#[allow(clippy::expect_used)]
fn temp_repo_with_debounce_config(name: &str) -> TempRepo {
    let repo = TempRepo::new("godharness-cli-test-repo", name);
    std::fs::write(
        repo.path.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nreinject-after-prompts: 3\n",
    )
    .expect("write godharness.yaml");
    repo
}

#[test]
fn session_state_persists_across_separate_process_invocations() {
    let repo = temp_repo_with_debounce_config("debounce");
    let session = SessionState::new("persist");
    let stdin_json = format!(
        r#"{{"prompt":"add a credential to config","session_id":"{}"}}"#,
        session.session_id
    );

    let first = run_adapter_hook("user-prompt-submit", &stdin_json, Some(&repo.path));
    assert!(
        !first.trim().is_empty(),
        "the first matching prompt should inject"
    );

    let second = run_adapter_hook("user-prompt-submit", &stdin_json, Some(&repo.path));
    assert!(
        second.trim().is_empty(),
        "a second separate process for the same session should see the debounce state \
         the first process persisted to disk, not start fresh"
    );
}
