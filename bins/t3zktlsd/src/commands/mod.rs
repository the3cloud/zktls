use anyhow::Result;
use clap::{Parser, Subcommand};

mod init;
mod node;
mod register_prover;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: SubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    /// Init config file
    Init(init::Cmd),

    /// Running prove node
    Node(node::Cmd),

    /// Register prover on the gateway
    RegisterProver(register_prover::Cmd),
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        match self.subcommand {
            SubCmd::Init(c) => c.execute().await,
            SubCmd::Node(cmd) => cmd.execute().await,
            SubCmd::RegisterProver(cmd) => cmd.execute().await,
        }
    }
}
