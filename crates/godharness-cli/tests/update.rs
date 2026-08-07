use std::process::Stdio;

fn godharness() -> std::process::Command {
    std::process::Command::new(env!("CARGO_BIN_EXE_godharness"))
}

struct TempRepo {
    path: std::path::PathBuf,
}

impl TempRepo {
    #[allow(clippy::expect_used)]
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "godharness-cli-test-update-{name}-{}",
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
    let repo = TempRepo::new("resync");
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
