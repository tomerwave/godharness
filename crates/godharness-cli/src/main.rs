use clap::{Parser, Subcommand};

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
    /// Scaffold godharness.yaml and starter standards. Human-facing, run once per repository.
    Init,
    /// Validate document schema, links, classification, selectors, and adapter configuration.
    /// Human- and CI-facing.
    Check,
    /// Resolve the standards relevant to a prompt or a set of changed paths, as JSON on stdout.
    /// Adapter-facing: agent integrations call this, humans do not run it directly.
    Context {
        /// The prompt text to match against standard keywords.
        #[arg(long)]
        prompt: Option<String>,
        /// One or more changed-file paths to match against standard path globs.
        #[arg(long = "paths", num_args = 1..)]
        paths: Vec<String>,
    },
    /// Validate the local installation and adapter wiring. Human- and CI-facing.
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            println!("godharness init: not yet implemented");
        }
        Command::Check => {
            println!("godharness check: not yet implemented");
        }
        Command::Context { prompt, paths } => {
            let config = godharness_core::Config {
                version: 1,
                suites: Vec::new(),
                standards: Vec::new(),
                adapters: Default::default(),
            };
            let resolved = godharness_core::resolve(&config, prompt.as_deref(), &paths);
            let json =
                serde_json::to_string(&resolved.standards).unwrap_or_else(|_| "[]".to_string());
            println!("{json}");
        }
        Command::Doctor => {
            println!("godharness doctor: not yet implemented");
        }
    }
}
