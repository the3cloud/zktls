use alloy::primitives::B256;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use t3zktls_cli_generator::StdinGenerator;
use t3zktls_prover::ZkTLSProver;
use t3zktls_submiter_ethereum::ZkTLSSubmiter;

use crate::{config::Config, downloader::build_sp1_prover};

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    #[arg(long, env)]
    mock_prover: bool,

    #[arg(long, env)]
    mock_submiter: bool,

    #[arg(short, long, env)]
    private_key: B256,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let mut config: Config = toml::from_str(&std::fs::read_to_string(self.config)?)?;

        // TODO: Add r0
        let (guest, file, pvkey) = build_sp1_prover(self.mock_prover).await?;

        config.prover.pvkey = pvkey;
        config.prover.guest_program_path = file;

        let input_builder = t3zktls_input_builder::TLSInputBuilder::new(config.input_builder)?;

        let generator = StdinGenerator::default();

        let submiter =
            ZkTLSSubmiter::new(config.submiter.build_local_config(self.private_key)).await?;

        let submitter = if self.mock_submiter {
            None
        } else {
            Some(submiter)
        };

        let mut prover =
            ZkTLSProver::new(config.prover, generator, input_builder, guest, submitter).await?;

        prover.run().await?;

        Ok(())
    }
}
