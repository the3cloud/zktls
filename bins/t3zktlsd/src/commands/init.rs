use std::path::PathBuf;

use alloy::primitives::{Address, B256};
use anyhow::Result;
use clap::Parser;
use tokio::fs;
use toml_edit::DocumentMut;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    #[arg(short, long, env)]
    gateway_address: Address,

    #[arg(short, long, env)]
    rpc_url: String,

    #[arg(long, env)]
    confirmations: Option<u64>,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let prover_id: alloy::primitives::FixedBytes<32> = B256::random();

        let config_str = include_str!("../../assets/config.toml");

        let mut doc = config_str.parse::<DocumentMut>()?;

        doc["prover"]["prover_id"] = prover_id.to_string().into();
        doc["submiter"]["gateway_address"] = self.gateway_address.to_string().into();
        doc["submiter"]["rpc_url"] = self.rpc_url.into();

        let confirmations = if let Some(confirmations) = self.confirmations {
            confirmations as i64
        } else {
            1
        };

        doc["submiter"]["confirmations"] = confirmations.into();

        let res = doc.to_string();

        let config: Config = toml::from_str(&res)?;

        log::debug!("config: {:#?}", config);

        if self.config.exists() {
            log::warn!("config file already exists");
            return Ok(());
        } else {
            fs::write(self.config, res).await?;
        }

        log::info!("init config success");

        Ok(())
    }
}
