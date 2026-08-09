use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use godharness_core::{ClaudeCodeEvent, SessionState};

#[derive(Parser)]
#[command(
    name = "godharness",
    version,
    about = "Agent-context and documentation-governance framework."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(
        about = "Scaffold godharness.yaml and starter standards. Human-facing, run once per repository."
    )]
    Init,
    #[command(
        about = "Validate document schema, links, classification, selectors, and adapter configuration. Human- and CI-facing."
    )]
    Check,
    #[command(
        about = "Resolve the standards relevant to a prompt or a set of changed paths, as JSON on stdout. Adapter-facing: agent integrations call this, humans do not run it directly."
    )]
    Context {
        #[arg(long, help = "The prompt text to match against standard keywords.")]
        prompt: Option<String>,
        #[arg(
            long = "paths",
            num_args = 1..,
            help = "One or more changed-file paths to match against standard path globs."
        )]
        paths: Vec<String>,
    },
    #[command(about = "Validate the local installation and adapter wiring. Human- and CI-facing.")]
    Doctor,
    #[command(
        about = "Respond to a live agent-tool hook event on stdin/stdout. Adapter-facing: invoked by generated hook configuration, not run by hand.",
        hide = true
    )]
    AdapterHook {
        #[arg(value_enum)]
        tool: AdapterTool,
        #[arg(long, value_enum)]
        event: AdapterHookEvent,
    },
    #[command(about = "Manage tool adapters for this repository. Human-facing.")]
    Adapters {
        #[command(subcommand)]
        action: AdaptersCommand,
    },
    #[command(
        about = "Sync this repository's suite pins and adapter config to the installed godharness version. Human- and CI-facing."
    )]
    Update,
    #[command(
        about = "Show which standards and skills have actually been injected into this repository's sessions, and their approximate token cost. Human-facing."
    )]
    Stats {
        #[arg(long, help = "Print as JSON instead of a table.")]
        json: bool,
        #[arg(long, help = "Clear the recorded usage log for this repository.")]
        reset: bool,
        #[arg(
            long,
            help = "Price every recorded token as if it ran on this model instead of the model auto-detected per event."
        )]
        model: Option<String>,
    },
}

#[derive(Subcommand)]
enum AdaptersCommand {
    #[command(
        about = "Wire godharness's live-hook adapter into the given tool's config. Human-facing, safe to rerun."
    )]
    Enable {
        #[arg(value_enum)]
        tool: AdapterTool,
    },
}

#[derive(Clone, ValueEnum)]
enum AdapterTool {
    ClaudeCode,
    Codex,
}

impl AdapterTool {
    fn cli_arg(&self) -> &'static str {
        match self {
            AdapterTool::ClaudeCode => "claude-code",
            AdapterTool::Codex => "codex",
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum AdapterHookEvent {
    UserPromptSubmit,
    SessionStart,
}

impl From<AdapterHookEvent> for ClaudeCodeEvent {
    fn from(event: AdapterHookEvent) -> Self {
        match event {
            AdapterHookEvent::UserPromptSubmit => ClaudeCodeEvent::UserPromptSubmit,
            AdapterHookEvent::SessionStart => ClaudeCodeEvent::SessionStart,
        }
    }
}

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

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

fn detect_current_model(transcript_path: Option<&str>) -> Option<String> {
    let contents = std::fs::read_to_string(transcript_path?).ok()?;
    contents.lines().rev().find_map(model_from_transcript_line)
}

fn record_usage(
    standards: &[godharness_core::ResolvedStandard],
    skills: &[godharness_core::ResolvedSkill],
    model: Option<&str>,
) {
    let Some(path) = usage_log_path_for_cwd() else {
        return;
    };
    let timestamp_unix = now_unix();
    let mut events: Vec<godharness_core::UsageEvent> = Vec::new();
    for standard in standards {
        events.push(godharness_core::UsageEvent {
            timestamp_unix,
            kind: godharness_core::UsageKind::Standard,
            id: standard.id.clone(),
            approx_tokens: godharness_core::approx_tokens(&standard.rule),
            model: model.map(str::to_string),
        });
    }
    for skill in skills {
        events.push(godharness_core::UsageEvent {
            timestamp_unix,
            kind: godharness_core::UsageKind::Skill,
            id: skill.id.clone(),
            approx_tokens: godharness_core::approx_tokens(&skill.description),
            model: model.map(str::to_string),
        });
    }
    let _ = godharness_core::append_events(&path, &events);
}

fn run_init() -> ExitCode {
    match godharness_core::run_init(&current_dir()) {
        Ok(report) => {
            println!(
                "godharness init: godharness.yaml {}, docs/godharness/example.md {}",
                if report.config_created {
                    "created"
                } else {
                    "already exists"
                },
                if report.starter_standard_created {
                    "created"
                } else {
                    "already exists"
                },
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("godharness init: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_check() -> ExitCode {
    match godharness_core::run_check(&current_dir()) {
        Ok(report) => {
            println!(
                "godharness check: {} standards, no problems",
                report.standard_count
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("godharness check: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_context(prompt: Option<String>, paths: Vec<String>) -> ExitCode {
    let graph = match godharness_core::load_repository_graph(&current_dir()) {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("godharness context: {error}");
            return ExitCode::FAILURE;
        }
    };
    let resolved = godharness_core::resolve(&graph, prompt.as_deref(), &paths);
    let json = serde_json::to_string(&resolved).unwrap_or_else(|_| "[]".to_string());
    println!("{json}");
    ExitCode::SUCCESS
}

fn run_doctor() -> ExitCode {
    match godharness_core::run_doctor(&current_dir()) {
        Ok(report) => {
            println!(
                "godharness doctor: {} standards, adapters enabled: {}",
                report.standard_count,
                if report.enabled_adapters.is_empty() {
                    "none".to_string()
                } else {
                    report.enabled_adapters.join(", ")
                }
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("godharness doctor: {error}");
            ExitCode::FAILURE
        }
    }
}

fn stdin_field(input: &str, field: &str) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_str(input).ok()?;
    parsed
        .get(field)
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn session_state_path(session_id: &str) -> PathBuf {
    std::env::temp_dir()
        .join("godharness")
        .join("sessions")
        .join(format!("{session_id}.json"))
}

fn load_session_state(session_id: &str) -> SessionState {
    std::fs::read_to_string(session_state_path(session_id))
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

fn save_session_state(session_id: &str, state: &SessionState) {
    let path = session_state_path(session_id);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(contents) = serde_json::to_string(state) {
        let _ = std::fs::write(path, contents);
    }
}

struct HookInputs {
    prompt: Option<String>,
    session_id: Option<String>,
    transcript_path: Option<String>,
}

fn read_hook_inputs(event: AdapterHookEvent) -> HookInputs {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);
    let prompt = match event {
        AdapterHookEvent::UserPromptSubmit => stdin_field(&input, "prompt"),
        AdapterHookEvent::SessionStart => None,
    };
    HookInputs {
        prompt,
        session_id: stdin_field(&input, "session_id"),
        transcript_path: stdin_field(&input, "transcript_path"),
    }
}

fn load_config_or_default(root: &std::path::Path) -> godharness_core::Config {
    godharness_core::load_config(root).unwrap_or_else(|_| godharness_core::Config {
        version: 1,
        suites: Vec::new(),
        standards: Vec::new(),
        adapters: Default::default(),
        reinject_after_prompts: 0,
    })
}

fn finish_hook(inputs: &HookInputs, state: &SessionState, result: godharness_core::HookResult) {
    if let Some(session_id) = inputs.session_id.as_deref() {
        save_session_state(session_id, state);
    }
    let model = detect_current_model(inputs.transcript_path.as_deref());
    record_usage(&result.standards, &result.skills, model.as_deref());
    if let Some(response) = result.response {
        println!("{response}");
    }
}

fn run_adapter_hook(_tool: AdapterTool, event: AdapterHookEvent) -> ExitCode {
    let inputs = read_hook_inputs(event);

    let root = current_dir();
    let Ok(graph) = godharness_core::load_repository_graph(&root) else {
        return ExitCode::SUCCESS;
    };
    let config = load_config_or_default(&root);
    let skills = godharness_core::load_suite_skills(&config);
    let mut state = inputs
        .session_id
        .as_deref()
        .map(load_session_state)
        .unwrap_or_default();

    let request = godharness_core::HookRequest {
        event: event.into(),
        prompt: inputs.prompt.as_deref(),
        reinject_after_prompts: config.reinject_after_prompts,
    };
    let result = godharness_core::claude_code_hook_response(&graph, &skills, request, &mut state);
    finish_hook(&inputs, &state, result);

    ExitCode::SUCCESS
}

fn run_adapters_enable(tool: AdapterTool) -> ExitCode {
    let root = current_dir();
    match godharness_core::enable_adapter(&root, tool.cli_arg()) {
        Ok(report) => {
            println!(
                "godharness adapters enable {}: godharness.yaml {}, {} {}, {} skill(s) installed",
                tool.cli_arg(),
                if report.godharness_yaml_updated {
                    "updated"
                } else {
                    "already configured"
                },
                report.hook_config_path.display(),
                if report.hook_config_updated {
                    "updated"
                } else {
                    "already configured"
                },
                report.skills_installed.len(),
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("godharness adapters enable: {error}");
            ExitCode::FAILURE
        }
    }
}

fn usage_log_path_for_cwd() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let repo_root = current_dir().canonicalize().ok()?;
    Some(godharness_core::usage_log_path(&home, &repo_root))
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

fn reset_usage_log(path: &std::path::Path) -> ExitCode {
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

fn run_stats(json: bool, reset: bool, model_override: Option<String>) -> ExitCode {
    let Some(path) = usage_log_path_for_cwd() else {
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

fn run_update() -> ExitCode {
    match godharness_core::update_repository(&current_dir()) {
        Ok(report) => {
            println!(
                "godharness update: {} suite(s) updated, {} adapter(s) resynced",
                report.suites_updated.len(),
                report.adapters_resynced.len(),
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("godharness update: {error}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => run_init(),
        Command::Check => run_check(),
        Command::Context { prompt, paths } => run_context(prompt, paths),
        Command::Doctor => run_doctor(),
        Command::AdapterHook { tool, event } => run_adapter_hook(tool, event),
        Command::Adapters {
            action: AdaptersCommand::Enable { tool },
        } => run_adapters_enable(tool),
        Command::Update => run_update(),
        Command::Stats { json, reset, model } => run_stats(json, reset, model),
    }
}
