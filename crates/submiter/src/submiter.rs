use alloy::{network::Network, primitives::Address, providers::Provider, transports::Transport};
use anyhow::Result;
use t3zktls_contracts_ethereum::IZkTLSGateway;
use t3zktls_core::{ProveResponse, Submiter};

pub struct ZkTLSSubmiter<P, T, N> {
    gateway_address: Address,
    confirmations: u64,
    provider: P,
    _marker: std::marker::PhantomData<(T, N)>,
}

impl<P, T, N> ZkTLSSubmiter<P, T, N> {
    pub fn new(provider: P, gateway_address: Address, confirmations: u64) -> Self {
        Self {
            gateway_address,
            confirmations,
            provider,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P, T, N> Submiter for ZkTLSSubmiter<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    async fn submit(&mut self, prove_response: ProveResponse) -> Result<()> {
        Self::submit(self, prove_response).await?;

        Ok(())
    }
}

impl<P, T, N> ZkTLSSubmiter<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    async fn submit(&mut self, prove_response: ProveResponse) -> Result<()> {
        let contract = IZkTLSGateway::new(self.gateway_address, &self.provider);

        let receipt = contract
            .deliveryResponse(
                prove_response.request_id,
                prove_response.request_hash,
                prove_response.response_data,
                prove_response.proof,
            )
            .send()
            .await?
            .with_required_confirmations(self.confirmations)
            .get_receipt()
            .await?;

        log::info!("Submitted proof: {:?}", receipt);

        Ok(())
    }
}
