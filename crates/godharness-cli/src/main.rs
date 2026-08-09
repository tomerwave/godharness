use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use godharness_core::{ClaudeCodeEvent, SessionState};

mod stats;

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

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum AdapterHookEvent {
    UserPromptSubmit,
    SessionStart,
    Stop,
}

impl From<AdapterHookEvent> for ClaudeCodeEvent {
    fn from(event: AdapterHookEvent) -> Self {
        match event {
            AdapterHookEvent::UserPromptSubmit => ClaudeCodeEvent::UserPromptSubmit,
            AdapterHookEvent::SessionStart => ClaudeCodeEvent::SessionStart,
            AdapterHookEvent::Stop => unreachable!("Stop is handled before this conversion"),
        }
    }
}

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
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
        AdapterHookEvent::SessionStart | AdapterHookEvent::Stop => None,
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

fn finish_hook(
    root: &std::path::Path,
    inputs: &HookInputs,
    state: &SessionState,
    result: godharness_core::HookResult,
) {
    if let Some(session_id) = inputs.session_id.as_deref() {
        save_session_state(session_id, state);
    }
    let model = stats::detect_current_model(inputs.transcript_path.as_deref());
    stats::record_usage(stats::RecordUsage {
        root,
        standards: &result.standards,
        skills: &result.skills,
        model: model.as_deref(),
        session_id: inputs.session_id.as_deref(),
    });
    if let Some(response) = result.response {
        println!("{response}");
    }
}

fn run_adapter_hook(_tool: AdapterTool, event: AdapterHookEvent) -> ExitCode {
    let inputs = read_hook_inputs(event);
    let root = current_dir();
    if event == AdapterHookEvent::Stop {
        return stats::run_stop_hook(
            &root,
            inputs.session_id.as_deref(),
            inputs.transcript_path.as_deref(),
        );
    }

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
    finish_hook(&root, &inputs, &state, result);

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
        Command::Stats { json, reset, model } => {
            stats::run_stats(&current_dir(), json, reset, model)
        }
    }
}
