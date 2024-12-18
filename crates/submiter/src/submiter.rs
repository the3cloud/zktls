use std::{str::FromStr, sync::Arc};

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::{reqwest::Url, Client, Http},
};
use anyhow::Result;
use t3zktls_contracts_ethereum::ZkTLSGateway;
use t3zktls_core::Submiter;
use t3zktls_program_core::Response;

use crate::Config;

pub struct ZkTLSSubmiter {
    gateway_address: Address,
    confirmations: u64,
    provider: Arc<dyn Provider<Http<Client>, Ethereum>>,
}

impl ZkTLSSubmiter {
    pub async fn new(config: Config) -> Result<Self> {
        let signer = config.signer.signer().await?;
        let wallet = EthereumWallet::new(signer);

        let provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(Url::from_str(&config.rpc_url)?);

        let provider: Arc<dyn Provider<Http<Client>, Ethereum>> = Arc::new(provider);

        Ok(Self {
            gateway_address: config.gateway_address,
            confirmations: config.confirmations,
            provider,
        })
    }
}

impl Submiter for ZkTLSSubmiter {
    async fn submit(&mut self, response: Response) -> Result<()> {
        log::info!("Submitting proof: {:#?}", response);

        Self::_submit(self, response).await?;

        Ok(())
    }
}

impl ZkTLSSubmiter {
    async fn _submit(&mut self, response: Response) -> Result<()> {
        let provider = self.provider.clone();

        let contract = ZkTLSGateway::new(self.gateway_address, provider.root());

        let receipt = contract
            .deliverResponse(
                response.proof.into(),
                response.prover_id,
                response.request_id,
                response.client,
                response.dapp,
                response.max_gas_price.into(),
                response.max_gas_limit,
                response.response.into(),
            )
            .send()
            .await?
            .with_required_confirmations(self.confirmations)
            .get_receipt()
            .await?;

        log::info!("Submitted proof: {}", receipt.transaction_hash);

        Ok(())
    }
}
