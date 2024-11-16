use std::{fs, path::PathBuf};

use alloy::primitives::{Address, B256};
use anyhow::Result;
use clap::Parser;
use toml_edit::DocumentMut;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    #[arg(short, long, env)]
    gateway_address: Address,

    #[arg(short, long, env)]
    rpc_url: String,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let prover_id: alloy::primitives::FixedBytes<32> = B256::random();

        let config_str = include_str!("../../assets/config.toml");

        let mut doc = config_str.parse::<DocumentMut>()?;

        doc["listener"]["prover_id"] = prover_id.to_string().into();
        doc["listener"]["gateway_address"] = self.gateway_address.to_string().into();
        doc["prover"]["rpc_url"] = self.rpc_url.into();
        doc["submiter"]["gateway_address"] = self.gateway_address.to_string().into();

        let res = doc.to_string();

        fs::write(self.config, res)?;

        Ok(())
    }
}
