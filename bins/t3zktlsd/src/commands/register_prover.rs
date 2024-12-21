use std::path::PathBuf;

use alloy::primitives::{Address, B256};
use anyhow::{anyhow, Result};
use clap::Parser;
use t3zktls_contracts_ethereum::{MockVerifier, Sp1Verifier, ZkTLSGateway};
use t3zktls_submiter_ethereum::ZkTLSSubmiter;

use crate::{config::Config, downloader::build_sp1_prover};

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    /// private key of the prover
    ///
    /// If not provided, the private key will be read from the environment variable `PRIVATE_KEY`
    #[arg(short, long, env)]
    private_key: B256,

    #[arg(long, env, group = "verifier")]
    sp1: bool,

    #[arg(long, env, group = "verifier")]
    r0: bool,

    #[arg(long, env, group = "verifier")]
    mock: bool,

    /// ZkVM Verifier address
    ///
    /// Must set in SP1 and Risc0 mode
    #[arg(short, long, env)]
    verifier_address: Option<Address>,

    #[arg(short, long, env)]
    beneficiary: Address,

    /// Submitter address
    ///
    /// If no submitter address is provided, the submitter will be generated from the private key
    #[arg(short, long, env)]
    submitter_address: Option<Address>,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let config: Config = toml::from_str(&std::fs::read_to_string(self.config)?)?;

        let submiter =
            ZkTLSSubmiter::new(config.submiter.build_local_config(self.private_key)).await?;

        let gateway = ZkTLSGateway::new(submiter.gateway_address(), submiter.root_provider());

        if self.sp1 {
            let (_, _, pvkey) = build_sp1_prover(true).await?;

            let verifier_receipt = Sp1Verifier::deploy_builder(
                submiter.root_provider(),
                self.verifier_address
                    .ok_or(anyhow!("Must set SP1 Verifier address"))?,
                pvkey,
            )
            .send()
            .await?
            .with_required_confirmations(submiter.confirmations())
            .get_receipt()
            .await?;

            let verifier_address = verifier_receipt
                .contract_address
                .ok_or(anyhow::anyhow!("No contract address found"))?;
            let tx_hash = verifier_receipt.transaction_hash;

            log::info!(
                "Deployed SP1 verifier on: {} at tx: {}",
                verifier_address,
                tx_hash
            );

            let submitter_address = if let Some(submitter_address) = self.submitter_address {
                submitter_address
            } else {
                submiter.signer_address()
            };

            let res = gateway
                .registerProver(
                    config.prover.prover_id,
                    verifier_address,
                    submitter_address,
                    self.beneficiary,
                )
                .send()
                .await?
                .with_required_confirmations(submiter.confirmations())
                .get_receipt()
                .await?;

            let tx_hash = res.transaction_hash;
            log::info!("Registered prover at tx: {}", tx_hash);
        }

        // Deploy mock verifier
        if self.mock {
            let verifier_receipt = MockVerifier::deploy_builder(submiter.root_provider())
                .send()
                .await?
                .with_required_confirmations(submiter.confirmations())
                .get_receipt()
                .await?;

            let verifier_address = verifier_receipt
                .contract_address
                .ok_or(anyhow::anyhow!("No contract address found"))?;
            let tx_hash = verifier_receipt.transaction_hash;

            log::info!(
                "Deployed mock verifier on: {} at tx: {}",
                verifier_address,
                tx_hash
            );

            let submitter_address = if let Some(submitter_address) = self.submitter_address {
                submitter_address
            } else {
                submiter.signer_address()
            };

            let res = gateway
                .registerProver(
                    config.prover.prover_id,
                    verifier_address,
                    submitter_address,
                    self.beneficiary,
                )
                .send()
                .await?
                .with_required_confirmations(submiter.confirmations())
                .get_receipt()
                .await?;

            let tx_hash = res.transaction_hash;
            log::info!("Registered prover at tx: {}", tx_hash);
        }

        Ok(())
    }
}
