use clap::{Parser, Subcommand};

mod commands;
mod utils;
use commands::{ExportVerifierArgs, ProveArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a proof using the input request file
    Prove(ProveArgs),

    /// Export verifier for the target chain
    ExportVerifier(ExportVerifierArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Prove(args) => args.execute().await,
        Commands::ExportVerifier(args) => args.execute(),
    }
}
