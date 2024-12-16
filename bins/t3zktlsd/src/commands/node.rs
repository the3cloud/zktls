use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use t3zktls_cli_generator::StdinGenerator;
use t3zktls_guest_prover_sp1::SP1GuestProver;
use t3zktls_prover::ZkTLSProver;
use t3zktls_submiter_ethereum::ZkTLSSubmiter;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    #[arg(short, long, env)]
    mock: bool,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let config: Config = toml::from_str(&std::fs::read_to_string(self.config)?)?;

        let input_builder = t3zktls_input_builder::TLSInputBuilder::new(config.input_builder)?;

        let mut guest = SP1GuestProver::default();

        if self.mock {
            guest = guest.mock();
        }

        let generator = StdinGenerator::default();

        let submiter = ZkTLSSubmiter::new(config.submiter).await?;

        let mut prover =
            ZkTLSProver::new(config.prover, generator, input_builder, guest, submiter).await?;

        prover.run().await?;

        Ok(())
    }
}
