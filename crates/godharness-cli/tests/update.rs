use std::process::Stdio;

mod common;
use common::{TempRepo, godharness};

fn temp_repo(name: &str) -> TempRepo {
    TempRepo::new("godharness-cli-test-update", name)
}

#[allow(clippy::expect_used)]
fn run_update(cwd: &std::path::Path) -> std::process::Output {
    godharness()
        .arg("update")
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run godharness update")
}

#[test]
fn update_resyncs_an_enabled_adapter_via_the_real_binary() {
    let repo = temp_repo("resync");
    std::fs::write(
        repo.path.join("godharness.yaml"),
        "version: 1\nsuites: [recommended@1]\nadapters:\n  codex: true\n",
    )
    .expect("write config");

    let output = run_update(&repo.path);

    assert!(output.status.success());
    assert!(repo.path.join(".codex/hooks.json").exists());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1 adapter(s) resynced"));
}
