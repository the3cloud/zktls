use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
enum TargetChain {
    Evm,
    Solana,
    Sui,
    Aptos,
    Ton,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a proof using the input request file
    Prove {
        /// Path to the input request file
        #[arg(long)]
        input_request_file: PathBuf,

        /// Target chain for the proof
        #[arg(long, value_enum)]
        target_chain: TargetChain,

        /// Use mock mode
        #[arg(long)]
        mock: bool,
    },

    /// Export verifier for the target chain
    ExportVerifier {
        /// Target chain for the verifier
        #[arg(long, value_enum)]
        target_chain: TargetChain,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Prove {
            input_request_file,
            target_chain,
            mock,
        } => Ok(()),
        Commands::ExportVerifier { target_chain } => Ok(()),
    }
}
