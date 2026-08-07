use std::process::{Command, Stdio};

fn godharness() -> Command {
    Command::new(env!("CARGO_BIN_EXE_godharness"))
}

struct TempRepo {
    path: std::path::PathBuf,
}

impl TempRepo {
    #[allow(clippy::expect_used)]
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "godharness-cli-test-adapters-enable-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp repo root");
        Self { path }
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[allow(clippy::expect_used)]
fn run_adapters_enable(cwd: &std::path::Path, tool: &str) -> std::process::Output {
    godharness()
        .arg("adapters")
        .arg("enable")
        .arg(tool)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run godharness adapters enable")
}

#[test]
fn enable_claude_code_creates_settings_json_via_the_real_binary() {
    let repo = TempRepo::new("claude-code");

    let output = run_adapters_enable(&repo.path, "claude-code");

    assert!(output.status.success());
    assert!(repo.path.join(".claude/settings.json").exists());
    assert!(repo.path.join("godharness.yaml").exists());
}

#[test]
fn enable_codex_creates_hooks_json_via_the_real_binary() {
    let repo = TempRepo::new("codex");

    let output = run_adapters_enable(&repo.path, "codex");

    assert!(output.status.success());
    assert!(repo.path.join(".codex/hooks.json").exists());
}

#[test]
fn enabling_twice_via_the_real_binary_is_idempotent() {
    let repo = TempRepo::new("idempotent");

    run_adapters_enable(&repo.path, "claude-code");
    let second = run_adapters_enable(&repo.path, "claude-code");

    assert!(second.status.success());
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(stdout.contains("already configured"));
}
