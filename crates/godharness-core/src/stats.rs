use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageKind {
    Standard,
    Skill,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsageEvent {
    pub timestamp_unix: u64,
    pub kind: UsageKind,
    pub id: String,
    pub approx_tokens: u32,
    #[serde(default)]
    pub model: Option<String>,
}

pub fn approx_tokens(text: &str) -> u32 {
    ((text.chars().count() as f64) / 4.0).ceil() as u32
}

fn sanitize_path_component(path: &Path) -> String {
    path.to_string_lossy()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn usage_log_path(home: &Path, repo_root: &Path) -> PathBuf {
    home.join(".godharness")
        .join("usage")
        .join(format!("{}.jsonl", sanitize_path_component(repo_root)))
}

fn serialize_events(events: &[UsageEvent]) -> String {
    let mut lines = String::new();
    for event in events {
        if let Ok(line) = serde_json::to_string(event) {
            lines.push_str(&line);
            lines.push('\n');
        }
    }
    lines
}

pub fn append_events(path: &Path, events: &[UsageEvent]) -> std::io::Result<()> {
    if events.is_empty() {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(serialize_events(events).as_bytes())
}

pub fn read_events(path: &Path) -> Vec<UsageEvent> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StatEntry {
    pub kind: UsageKind,
    pub id: String,
    pub fires: u32,
    pub total_approx_tokens: u64,
}

pub fn aggregate(events: &[UsageEvent]) -> Vec<StatEntry> {
    use std::collections::BTreeMap;

    let mut totals: BTreeMap<(UsageKind, String), (u32, u64)> = BTreeMap::new();
    for event in events {
        let entry = totals
            .entry((event.kind, event.id.clone()))
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += u64::from(event.approx_tokens);
    }

    let mut entries: Vec<StatEntry> = totals
        .into_iter()
        .map(|((kind, id), (fires, total_approx_tokens))| StatEntry {
            kind,
            id,
            fires,
            total_approx_tokens,
        })
        .collect();
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.total_approx_tokens));
    entries
}
