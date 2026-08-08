use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
pub fn godharness() -> Command {
    Command::new(env!("CARGO_BIN_EXE_godharness"))
}

pub struct TempRepo {
    pub path: PathBuf,
}

impl TempRepo {
    #[allow(clippy::expect_used, dead_code)]
    pub fn new(prefix: &str, name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("{prefix}-{name}-{}", std::process::id()));
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
