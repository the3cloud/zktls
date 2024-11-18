use std::{path::PathBuf, str::FromStr};

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::B256,
    providers::ProviderBuilder,
    signers::local::LocalSigner,
    transports::http::{reqwest::Url, Client, Http},
};
use anyhow::Result;
use clap::Parser;
use t3zktls_kms_local::K256LocalDecryptorGenerator;
use t3zktls_listeners_ethereum::ZkTLSListener;
use t3zktls_prover::ZkTLSProver;
use t3zktls_sp1_guest_prover::SP1GuestProver;
use t3zktls_submiter_ethereum::ZkTLSSubmiter;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[arg(short, long, env)]
    config: PathBuf,

    #[arg(short, long, env)]
    private_key: B256,

    #[arg(short, long, env)]
    decryptor_private_key: B256,

    #[arg(short, long, env)]
    mock: bool,
}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        let config: Config = toml::from_str(&std::fs::read_to_string(self.config)?)?;

        let signer = LocalSigner::from_bytes(&self.private_key)?;

        let wallet = EthereumWallet::new(signer);

        let provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(Url::from_str(&config.prover.rpc_url)?);

        let decryptor = K256LocalDecryptorGenerator::new(self.decryptor_private_key.as_ref())?;

        let listener: ZkTLSListener<_, _, Http<Client>, Ethereum> =
            ZkTLSListener::new(config.listener, provider.clone(), decryptor);

        let input_builder = t3zktls_input_builder::TLSInputBuilder::new(config.input_builder)?;

        let mut guest = SP1GuestProver::default();

        if self.mock {
            guest = guest.mock();
        }

        let submiter: ZkTLSSubmiter<_, Http<Client>, Ethereum> =
            ZkTLSSubmiter::new(provider, config.submiter);

        let mut prover = ZkTLSProver::new(
            config.prover.prover,
            listener,
            input_builder,
            guest,
            submiter,
        );

        prover.run().await?;

        Ok(())
    }
}
