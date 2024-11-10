/// A listener for ZkTLS Gateway events.
///
/// This struct is responsible for listening to events emitted by the ZkTLS Gateway contract,
/// processing them, and handling TLS call requests.
use std::collections::HashMap;

use alloy::{
    consensus::Transaction,
    network::Network,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
    transports::Transport,
};
use anyhow::{anyhow, Result};
use t3zktls_contracts_ethereum::IZkTLSGateway::{
    RequestTLSCallBegin, RequestTLSCallSegment, RequestTLSCallTemplateField,
};
use t3zktls_core::{Listener, ProveRequest, TLSDataDecryptorGenerator};

use crate::{Config, RequestBuilder};

/// The main Listener struct.
pub struct ZkTLSListener<P, D, T, N> {
    provider: P,
    gateway_address: Address,

    prover_id: B256,

    begin_block_number: u64,
    block_number_batch_size: u64,

    decryptor: D,

    _marker: std::marker::PhantomData<(T, N)>,
}

impl<P, D, T, N> ZkTLSListener<P, D, T, N> {
    /// Creates a new Listener instance.
    ///
    /// # Arguments
    ///
    /// * `loop_number` - Optional number of loops to run. If None, runs indefinitely.
    /// * `config` - Configuration for the listener.
    /// * `provider` - The provider used to interact with the blockchain.
    /// * `handler` - The handler for processing TLS call requests.
    /// * `decryptor` - The decryptor for decoding TLS data.
    pub fn new(config: Config, provider: P, decryptor: D) -> Self {
        Self {
            provider,
            gateway_address: config.gateway_address,
            begin_block_number: config.begin_block_number,
            block_number_batch_size: config.block_number_batch_size,
            prover_id: config.prover_id,
            decryptor,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P, D, T, N> Listener for ZkTLSListener<P, D, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    D: TLSDataDecryptorGenerator + Send + Sync,
    D::Decryptor: Send + Sync,
{
    async fn pull(&mut self) -> Result<Vec<ProveRequest>> {
        self.pull_blocks().await
    }
}

impl<P, D, T, N> ZkTLSListener<P, D, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    D: TLSDataDecryptorGenerator,
{
    /// Pulls and processes blocks of events.
    async fn pull_blocks(&mut self) -> Result<Vec<ProveRequest>> {
        let latest_block_number = self.provider.get_block_number().await?;
        let filter = self.compute_log_filter(latest_block_number);

        if let Some(filter) = filter {
            log::trace!("filter: {:?}", filter);
            let logs = self.provider.get_logs(&filter).await?;

            let requests = self.tidy_logs_by_txid_and_build_requests(logs).await?;

            Ok(requests)
        } else {
            Ok(Vec::new())
        }
    }

    fn compute_log_filter(&mut self, latest_block_number: u64) -> Option<Filter> {
        let to_block_number = if latest_block_number < self.begin_block_number {
            return None;
        } else if latest_block_number < self.begin_block_number + self.block_number_batch_size {
            latest_block_number
        } else {
            self.begin_block_number + self.block_number_batch_size
        };

        let begin_block_number = self.begin_block_number;

        let topics = vec![
            RequestTLSCallBegin::SIGNATURE_HASH,
            RequestTLSCallSegment::SIGNATURE_HASH,
            RequestTLSCallTemplateField::SIGNATURE_HASH,
        ];

        let filter = Filter::new()
            .from_block(begin_block_number)
            .to_block(to_block_number)
            .address(self.gateway_address)
            .event_signature(topics);

        log::info!(
            "pulling block, from block number: {} to block number: {}",
            begin_block_number,
            to_block_number
        );

        self.begin_block_number = to_block_number + 1;

        Some(filter)
    }

    async fn tidy_logs_by_txid_and_build_requests(
        &mut self,
        logs: Vec<Log>,
    ) -> Result<Vec<ProveRequest>> {
        let mut grouped_logs: HashMap<B256, Vec<Log>> = HashMap::new();

        // Group logs by transaction hash
        for log in logs {
            if let Some(tx_hash) = log.transaction_hash {
                grouped_logs.entry(tx_hash).or_default().push(log);
            }
        }

        let mut requests = Vec::new();

        // Sort each group by log_index
        for logs in grouped_logs.values_mut() {
            logs.sort_by_key(|log| log.log_index);

            requests.extend(
                self.tidy_logs_by_request_id_and_build_requests(logs)
                    .await?,
            );
        }

        Ok(requests)
    }

    async fn tidy_logs_by_request_id_and_build_requests(
        &mut self,
        logs: &[Log],
    ) -> Result<Vec<ProveRequest>> {
        let mut grouped_logs: HashMap<B256, Vec<Log>> = HashMap::new();

        for log in logs {
            let request_id = log
                .topics()
                .get(1)
                .ok_or(anyhow::anyhow!("log topic is empty"))?;

            grouped_logs
                .entry(*request_id)
                .or_default()
                .push(log.clone());
        }

        let mut requests = Vec::new();

        for (_, mut logs) in grouped_logs {
            logs.sort_by_key(|log| log.log_index);

            let call_result = self.build_request(&logs).await?;

            if let Some(prove_request) = call_result {
                requests.push(prove_request);
            }
        }

        Ok(requests)
    }

    async fn build_request(&mut self, logs: &[Log]) -> Result<Option<ProveRequest>> {
        let mut builder = RequestBuilder::new(self.prover_id, &self.decryptor);

        for log in logs {
            let selector = log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?;

            match *selector {
                RequestTLSCallBegin::SIGNATURE_HASH => {
                    let decoded = RequestTLSCallBegin::decode_log_data(log.data(), false)?;

                    let request_template_hash = decoded.requestTemplateHash;
                    let response_template_hash = decoded.responseTemplateHash;

                    if request_template_hash != B256::ZERO {
                        let request_template = self
                            .provider
                            .get_transaction_by_hash(request_template_hash)
                            .await?
                            .ok_or(anyhow!("request template not found"))?;

                        let request_template_data = request_template.input();

                        builder.add_request_template(request_template_data)?;
                    }

                    if response_template_hash != B256::ZERO {
                        let response_template = self
                            .provider
                            .get_transaction_by_hash(response_template_hash)
                            .await?
                            .ok_or(anyhow!("response template not found"))?;

                        let response_template_data = response_template.input();

                        builder.add_response_template(response_template_data)?;
                    }

                    builder.add_request_from_begin_logs(decoded)?;
                }
                RequestTLSCallSegment::SIGNATURE_HASH => {
                    let decoded = RequestTLSCallSegment::decode_log_data(log.data(), false)?;
                    builder.add_request_from_segment_logs(decoded).await?;
                }
                RequestTLSCallTemplateField::SIGNATURE_HASH => {
                    let decoded = RequestTLSCallTemplateField::decode_log_data(log.data(), false)?;
                    builder
                        .add_request_from_template_field_logs(decoded)
                        .await?;
                }
                _ => {
                    return Err(anyhow::anyhow!("unknown log selector: {:#x}", selector));
                }
            }
        }

        let prove_request = builder.build()?;

        Ok(Some(prove_request))
    }
}

#[cfg(test)]
mod tests {
    use alloy::{
        primitives::{Bytes, B256},
        providers::ProviderBuilder,
    };
    use anyhow::Result;
    use t3zktls_contracts_ethereum::ZkTLSGateway;
    use t3zktls_core::{Listener, Request, TLSDataDecryptor, TLSDataDecryptorGenerator};

    use crate::{Config, ZkTLSListener};

    pub struct DefaultDecryptor;

    impl TLSDataDecryptorGenerator for DefaultDecryptor {
        type Decryptor = Self;

        async fn generate_decryptor(
            &self,
            _encrypted_public_key: &alloy::primitives::Bytes,
        ) -> Result<Self::Decryptor> {
            Ok(Self)
        }
    }

    impl TLSDataDecryptor for DefaultDecryptor {
        async fn decrypt_tls_data(&mut self, _data: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_original_request() {
        let _ = env_logger::builder().is_test(true).try_init();

        let provider = ProviderBuilder::new().on_anvil();

        let zk_tls_gateway = ZkTLSGateway::deploy(provider.clone()).await.unwrap();

        let gateway_address = *zk_tls_gateway.address();

        log::info!("Gateway address: {}", gateway_address);

        let config = Config {
            gateway_address,
            begin_block_number: 0,
            block_number_batch_size: 100,
            prover_id: B256::ZERO,
        };

        let _receipt = zk_tls_gateway
            .requestTLSCallSegment(
                "httpbin.org:443".into(),
                "httpbin.org".into(),
                Bytes::new(),
                vec![
                    b"GET /get HTTP/1.1\r\n".to_vec().into(),
                    b"Host: httpbin.org\r\n".to_vec().into(),
                    b"Connection: close\r\n\r\n".to_vec().into(),
                ],
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        let mut listener = ZkTLSListener::new(config, provider, DefaultDecryptor);

        let res = listener.pull().await.unwrap();

        if let Request::Original(original_request) = &res[0].request {
            let data = original_request.data.to_vec();
            log::info!("original_request: {:?}", String::from_utf8(data).unwrap());
        }

        log::info!("requests: {:#?}", res);
    }
}
