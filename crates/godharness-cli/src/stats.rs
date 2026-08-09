use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn model_from_transcript_line(line: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let is_assistant = value.get("type").and_then(|t| t.as_str()) == Some("assistant");
    if !is_assistant {
        return None;
    }
    value
        .get("message")?
        .get("model")?
        .as_str()
        .map(str::to_string)
}

pub fn detect_current_model(transcript_path: Option<&str>) -> Option<String> {
    let contents = std::fs::read_to_string(transcript_path?).ok()?;
    contents.lines().rev().find_map(model_from_transcript_line)
}

fn usage_log_path_for_root(root: &Path) -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let repo_root = root.canonicalize().ok()?;
    Some(godharness_core::usage_log_path(&home, &repo_root))
}

pub struct RecordUsage<'a> {
    pub root: &'a Path,
    pub standards: &'a [godharness_core::ResolvedStandard],
    pub skills: &'a [godharness_core::ResolvedSkill],
    pub model: Option<&'a str>,
    pub session_id: Option<&'a str>,
}

pub fn record_usage(call: RecordUsage) {
    let Some(path) = usage_log_path_for_root(call.root) else {
        return;
    };
    let timestamp_unix = now_unix();
    let mut events: Vec<godharness_core::UsageEvent> = Vec::new();
    for standard in call.standards {
        events.push(godharness_core::UsageEvent {
            timestamp_unix,
            kind: godharness_core::UsageKind::Standard,
            id: standard.id.clone(),
            approx_tokens: godharness_core::approx_tokens(&standard.rule),
            model: call.model.map(str::to_string),
            session_id: call.session_id.map(str::to_string),
        });
    }
    for skill in call.skills {
        events.push(godharness_core::UsageEvent {
            timestamp_unix,
            kind: godharness_core::UsageKind::Skill,
            id: skill.id.clone(),
            approx_tokens: godharness_core::approx_tokens(&skill.description),
            model: call.model.map(str::to_string),
            session_id: call.session_id.map(str::to_string),
        });
    }
    let _ = godharness_core::append_events(&path, &events);
}

pub fn run_stop_hook(
    root: &Path,
    session_id: Option<&str>,
    transcript_path: Option<&str>,
) -> ExitCode {
    let (Some(session_id), Some(path)) = (session_id, usage_log_path_for_root(root)) else {
        return ExitCode::SUCCESS;
    };
    if let Some(model) = detect_current_model(transcript_path) {
        let _ = godharness_core::backfill_model(&path, session_id, &model);
    }
    ExitCode::SUCCESS
}

fn print_stats_table(entries: &[godharness_core::StatEntry]) {
    if entries.is_empty() {
        println!("godharness stats: no recorded usage yet for this repository");
        return;
    }

    let total_tokens: u64 = entries.iter().map(|entry| entry.total_approx_tokens).sum();
    let total_fires: u32 = entries.iter().map(|entry| entry.fires).sum();

    println!(
        "{:<10} {:<40} {:>8} {:>18}",
        "kind", "id", "fires", "approx tokens"
    );
    for entry in entries {
        let kind = match entry.kind {
            godharness_core::UsageKind::Standard => "standard",
            godharness_core::UsageKind::Skill => "skill",
        };
        println!(
            "{:<10} {:<40} {:>8} {:>18}",
            kind, entry.id, entry.fires, entry.total_approx_tokens
        );
    }
    println!(
        "\n~{total_tokens} approx tokens spent on injected context across {total_fires} recorded firing(s)"
    );
}

fn print_cost_report(report: &godharness_core::CostReport, model_override: Option<&str>) {
    if report.priced.is_empty() && report.unpriced_tokens == 0 {
        return;
    }

    match model_override {
        Some(model) => println!("\nHypothetical cost if every token ran on {model}:"),
        None => println!("\nEstimated cost by auto-detected model:"),
    }
    for entry in &report.priced {
        println!(
            "  {:<30} {:>10} tokens   ~${:.4}",
            entry.model, entry.tokens, entry.estimated_usd
        );
    }
    if report.unpriced_tokens > 0 {
        println!(
            "  {:<30} {:>10} tokens   (no model detected or no matching price - excluded)",
            "(unpriced)", report.unpriced_tokens
        );
    }
    println!(
        "Pricing: {} snapshot from {} ({}). Input-token rate only, estimate only - check your actual provider invoice.",
        godharness_core::snapshot_source(),
        godharness_core::snapshot_fetched_at(),
        if model_override.is_some() {
            "hypothetical override"
        } else {
            "per-event auto-detected model"
        }
    );
}

fn reset_usage_log(path: &Path) -> ExitCode {
    let _ = std::fs::remove_file(path);
    println!("godharness stats: usage log cleared");
    ExitCode::SUCCESS
}

fn print_stats_json(
    entries: &[godharness_core::StatEntry],
    cost: &godharness_core::CostReport,
) -> ExitCode {
    let output = serde_json::json!({ "entries": entries, "cost": cost });
    println!(
        "{}",
        serde_json::to_string(&output).unwrap_or_else(|_| "{}".to_string())
    );
    ExitCode::SUCCESS
}

pub fn run_stats(root: &Path, json: bool, reset: bool, model_override: Option<String>) -> ExitCode {
    let Some(path) = usage_log_path_for_root(root) else {
        eprintln!("godharness stats: could not resolve a home directory");
        return ExitCode::FAILURE;
    };
    if reset {
        return reset_usage_log(&path);
    }

    let events = godharness_core::read_events(&path);
    let entries = godharness_core::aggregate(&events);
    let cost = godharness_core::estimate_cost(&events, model_override.as_deref());
    if json {
        return print_stats_json(&entries, &cost);
    }

    print_stats_table(&entries);
    print_cost_report(&cost, model_override.as_deref());
    ExitCode::SUCCESS
}
