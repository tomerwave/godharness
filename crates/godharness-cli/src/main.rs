use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use godharness_core::ClaudeCodeEvent;

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
}

#[derive(Clone, ValueEnum)]
enum AdapterTool {
    ClaudeCode,
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

fn prompt_from_stdin(input: &str) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_str(input).ok()?;
    parsed
        .get("prompt")
        .and_then(|prompt| prompt.as_str())
        .map(str::to_string)
}

fn run_adapter_hook(tool: AdapterTool, event: AdapterHookEvent) -> ExitCode {
    let AdapterTool::ClaudeCode = tool;

    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);

    let prompt = match event {
        AdapterHookEvent::UserPromptSubmit => prompt_from_stdin(&input),
        AdapterHookEvent::SessionStart => None,
    };

    let Ok(graph) = godharness_core::load_repository_graph(&current_dir()) else {
        return ExitCode::SUCCESS;
    };

    if let Some(response) =
        godharness_core::claude_code_hook_response(&graph, event.into(), prompt.as_deref())
    {
        println!("{response}");
    }

    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => run_init(),
        Command::Check => run_check(),
        Command::Context { prompt, paths } => run_context(prompt, paths),
        Command::Doctor => run_doctor(),
        Command::AdapterHook { tool, event } => run_adapter_hook(tool, event),
    }
}
