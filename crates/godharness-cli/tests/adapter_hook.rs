use std::io::Write;
use std::process::{Command, Stdio};

fn godharness() -> Command {
    Command::new(env!("CARGO_BIN_EXE_godharness"))
}

#[allow(clippy::expect_used)]
fn run_adapter_hook(event: &str, stdin_json: &str, cwd: Option<&std::path::Path>) -> String {
    let mut command = godharness();
    command
        .arg("adapter-hook")
        .arg("claude-code")
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

    let parsed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout should be valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "UserPromptSubmit"
    );
    let context = parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string");
    assert!(context.contains("secrets-and-security"));
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

    let parsed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout should be valid JSON");
    assert_eq!(
        parsed["hookSpecificOutput"]["hookEventName"],
        "SessionStart"
    );
    let context = parsed["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .expect("additionalContext should be a string");
    assert!(context.contains("secrets-and-security"));
}

struct TempRepo {
    path: std::path::PathBuf,
}

impl TempRepo {
    #[allow(clippy::expect_used)]
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "godharness-cli-test-repo-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp repo root");
        std::fs::write(
            path.join("godharness.yaml"),
            "version: 1\nsuites: [recommended@1]\nreinject-after-prompts: 3\n",
        )
        .expect("write godharness.yaml");
        Self { path }
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn session_state_persists_across_separate_process_invocations() {
    let repo = TempRepo::new("debounce");
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
