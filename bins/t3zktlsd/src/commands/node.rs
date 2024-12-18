use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

use alloy::{hex::FromHex, primitives::B256};
use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use t3zktls_cli_generator::StdinGenerator;
use t3zktls_core::ZkProver;
use t3zktls_guest_prover_sp1::SP1GuestProver;
use t3zktls_prover::ZkTLSProver;
use t3zktls_submiter_ethereum::ZkTLSSubmiter;
use tokio::fs;

use crate::config::Config;

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

async fn download_program(url: &str, path: &Path) -> Result<()> {
    let res = reqwest::get(url).await?;
    let mut file = std::fs::File::create(path)?;

    let mut bytes_stream = res.bytes_stream();

    while let Some(chunk) = bytes_stream.next().await {
        file.write_all(&chunk?)?;
    }

    Ok(())
}

async fn build_sp1_prover(path: PathBuf, mock: bool) -> Result<(impl ZkProver, PathBuf, B256)> {
    let mut guest = SP1GuestProver::default();
    let file = path.join("zktls-sp1");

    if mock {
        guest = guest.mock();
    }

    let url = "https://github.com/the3cloud/zkvm-programs/releases/download/v0.1.0-alpha/zktls-sp1";

    if !file.exists() {
        log::info!("downloading program from {}", url);
        download_program(url, &file).await?;
        log::info!("downloaded program success {}", file.display());
    } else {
        log::info!(
            "program already exists {}, if you want to download again, please remove it",
            file.display()
        );
    }

    let pvkey =
        B256::from_hex("0x002c1167a4d8dd15018ac2d333a23e21f6aeaf0e28ff93ad67926588b26fccd4")?;

    Ok((guest, file, pvkey))
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let mut config: Config = toml::from_str(&std::fs::read_to_string(self.config)?)?;

        let dir = env::var("HOME")?;
        let dir = Path::new(&dir).join(".local").join("t3zktlsd");

        fs::create_dir_all(&dir).await?;

        // TODO: Add r0
        let (guest, file, pvkey) = build_sp1_prover(dir, self.mock_prover).await?;

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
