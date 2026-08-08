use std::process::Stdio;

mod common;
use common::{TempRepo, godharness};

fn temp_repo(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-adapters-enable", name)
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
    let repo = temp_repo("claude-code");

    let output = run_adapters_enable(&repo.path, "claude-code");

    assert!(output.status.success());
    assert!(repo.path.join(".claude/settings.json").exists());
    assert!(repo.path.join("godharness.yaml").exists());
}

#[test]
fn enable_codex_creates_hooks_json_via_the_real_binary() {
    let repo = temp_repo("codex");

    let output = run_adapters_enable(&repo.path, "codex");

    assert!(output.status.success());
    assert!(repo.path.join(".codex/hooks.json").exists());
}

#[test]
fn enabling_twice_via_the_real_binary_is_idempotent() {
    let repo = temp_repo("idempotent");

    run_adapters_enable(&repo.path, "claude-code");
    let second = run_adapters_enable(&repo.path, "claude-code");

    assert!(second.status.success());
    let stdout = String::from_utf8_lossy(&second.stdout);
    assert!(stdout.contains("already configured"));
}

#[test]
fn enable_claude_code_installs_skills_via_the_real_binary() {
    let repo = temp_repo("skills");

    let output = run_adapters_enable(&repo.path, "claude-code");

    assert!(output.status.success());
    assert!(
        repo.path
            .join(".claude/skills/atomic-commits/SKILL.md")
            .exists()
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("15 skill(s) installed"));
}
