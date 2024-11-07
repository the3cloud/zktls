/// A listener for ZkTLS Gateway events.
///
/// This struct is responsible for listening to events emitted by the ZkTLS Gateway contract,
/// processing them, and handling TLS call requests.
use std::{collections::HashMap, thread::sleep, time::Duration};

use alloy::{
    consensus::Transaction,
    network::Network,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
    transports::Transport,
};
use anyhow::Result;
use t3zktls_contracts_ethereum::ZkTLSGateway::{
    RequestTLSCallBegin, RequestTLSCallSegment, RequestTLSCallTemplateField,
};
use t3zktls_core::{
    ProveRequest, RequestTLSCallHandler, RequestTemplate, TLSDataDecryptor,
    TLSDataDecryptorGenerator,
};

use crate::Config;

/// The main Listener struct.
pub struct Listener<P, H, D, T, N> {
    provider: P,
    gateway_address: Address,

    prover_id: B256,

    begin_block_number: u64,
    block_number_batch_size: u64,

    sleep_duration: Duration,

    loop_number: Option<u64>,

    handler: H,
    decryptor: D,

    _marker: std::marker::PhantomData<(T, N)>,
}

impl<P, H, D, T, N> Listener<P, H, D, T, N> {
    /// Creates a new Listener instance.
    ///
    /// # Arguments
    ///
    /// * `loop_number` - Optional number of loops to run. If None, runs indefinitely.
    /// * `config` - Configuration for the listener.
    /// * `provider` - The provider used to interact with the blockchain.
    /// * `handler` - The handler for processing TLS call requests.
    /// * `decryptor` - The decryptor for decoding TLS data.
    pub fn new(
        loop_number: Option<u64>,
        config: Config,
        provider: P,
        handler: H,
        decryptor: D,
    ) -> Self {
        Self {
            provider,
            gateway_address: config.gateway_address,
            begin_block_number: config.begin_block_number,
            block_number_batch_size: config.block_number_batch_size,
            prover_id: config.prover_id,
            sleep_duration: Duration::from_secs(config.sleep_duration),
            loop_number,
            handler,
            decryptor,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P, H, D, T, N> Listener<P, H, D, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
    H: RequestTLSCallHandler,
    D: TLSDataDecryptorGenerator,
{
    /// Pulls and processes a single block of events.
    async fn pull_block(&mut self) -> Result<()> {
        let latest_block_number = self.provider.get_block_number().await?;
        let filter = self.compute_log_filter(latest_block_number);

        if let Some(filter) = filter {
            let logs = self.provider.get_logs(&filter).await?;

            self.tidy_logs_and_call(logs).await?;
        }

        Ok(())
    }
    /// Pulls and processes blocks of events.
    ///
    /// This method is the main entry point for the listener. It continuously pulls blocks
    /// and processes the events within them. The behavior depends on whether a loop number
    /// is specified:
    ///
    /// - If `loop_number` is Some, it will process that many blocks and then stop.
    /// - If `loop_number` is None, it will run indefinitely, processing blocks as they come.
    ///
    /// After processing each block, the method sleeps for the duration specified in the
    /// configuration to avoid overwhelming the network.
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` which is Ok if all blocks were processed successfully,
    /// or an Error if any issues occurred during processing.
    pub async fn pull_blocks(&mut self) -> Result<()> {
        if let Some(loop_number) = &mut self.loop_number {
            for _ in 0..*loop_number {
                self.pull_block().await?;

                sleep(self.sleep_duration);
            }

            Ok(())
        } else {
            loop {
                self.pull_block().await?;

                sleep(self.sleep_duration);
            }
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

    /// Computes the call data from a set of logs.
    async fn compute_call(&self, logs: &[Log]) -> Result<Option<ProveRequest>> {
        let mut request = ProveRequest::default();

        let mut request_data = Vec::new();

        let mut decryptor = None;

        let mut request_template = None;

        for log in logs {
            if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallBegin::SIGNATURE_HASH
            {
                let decoded = RequestTLSCallBegin::decode_log_data(log.data(), false)?;

                if decoded.prover != self.prover_id {
                    return Ok(None);
                }

                request.request_id = decoded.requestId;
                request.prover_id = decoded.prover;
                request.remote.url = decoded.remote;
                request.remote.server_name = decoded.serverName;
                request.max_response_size = decoded.maxResponseSize;

                let request_template_hash = decoded.requestTemplateHash;
                request.response_template_id = request_template_hash;

                if request_template_hash != B256::ZERO {
                    let data = self
                        .provider
                        .get_transaction_by_hash(request_template_hash)
                        .await?
                        .ok_or(anyhow::anyhow!(
                            "Target transaction is not found, request template hash: {}",
                            request_template_hash
                        ))?;

                    let input = data.input().clone();

                    request_template = Some(RequestTemplate::new(input)?);
                }

                let response_template_hash = decoded.responseTemplateHash;

                if response_template_hash != B256::ZERO {
                    let data = self
                        .provider
                        .get_transaction_by_hash(response_template_hash)
                        .await?
                        .ok_or(anyhow::anyhow!(
                            "Target transaction is not found, response template hash: {}",
                            response_template_hash
                        ))?;

                    let input = data.input().clone();

                    request.response_template = input;
                }

                decryptor = Some(
                    self.decryptor
                        .generate_decryptor(&decoded.encryptedKey)
                        .await?,
                );
            } else if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallSegment::SIGNATURE_HASH
            {
                if request_template.is_some() {
                    return Err(anyhow::anyhow!(
                        "request template is already set, it means wrong event."
                    ));
                }

                let decoded = RequestTLSCallSegment::decode_log_data(log.data(), false)?;

                if !decoded.isEncrypted {
                    request_data.extend_from_slice(&decoded.data);
                } else {
                    let mut dd = decoded.data.to_vec();

                    decryptor
                        .as_mut()
                        .ok_or(anyhow::anyhow!("decryptor is not initialized, the first event must be RequestTLSCallBegin"))?
                        .decrypt_tls_data(&mut dd)
                        .await?;
                    request_data.extend_from_slice(&dd);
                }
            } else if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallTemplateField::SIGNATURE_HASH
            {
                if let Some(ref mut request_template) = request_template {
                    let decoded = RequestTLSCallTemplateField::decode_log_data(log.data(), false)?;

                    let value = if decoded.isEncrypted {
                        let mut dd = decoded.value.to_vec();

                        decryptor
                            .as_mut()
                            .ok_or(anyhow::anyhow!(
                                "decryptor is not initialized, the first event must be RequestTLSCallBegin"
                            ))?
                            .decrypt_tls_data(&mut dd)
                            .await?;

                        dd
                    } else {
                        decoded.value.to_vec()
                    };

                    request_template.fill(&decoded.field, &value);
                } else {
                    return Err(anyhow::anyhow!(
                        "request template is not set, it means wrong event."
                    ));
                }
            }
        }

        if let Some(request_template) = request_template {
            request.request_data = request_template.finalize()?;
        } else {
            request.request_data = request_data.into();
        }

        Ok(Some(request))
    }

    async fn compute_calls(&mut self, logs: &[Log]) -> Result<()> {
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

        for (_, mut logs) in grouped_logs {
            logs.sort_by_key(|log| log.log_index);

            let call_result = self.compute_call(&logs).await;

            match call_result {
                Ok(None) => {
                    log::debug!("mismatch call");
                }
                Ok(Some(prove_request)) => {
                    log::trace!("prove request: {:#?}", prove_request);

                    self.handler.handle_request_tls_call(prove_request).await?;
                }
                Err(e) => {
                    log::error!("failed to compute call: {:?}", e);
                }
            }
        }

        Ok(())
    }

    /// Processes the logs and calls the handler for each TLS call request.
    async fn tidy_logs_and_call(&mut self, logs: Vec<Log>) -> Result<()> {
        let mut grouped_logs: HashMap<B256, Vec<Log>> = HashMap::new();

        // Group logs by transaction hash
        for log in logs {
            if let Some(tx_hash) = log.transaction_hash {
                grouped_logs.entry(tx_hash).or_default().push(log);
            }
        }

        // Sort each group by log_index
        for logs in grouped_logs.values_mut() {
            logs.sort_by_key(|log| log.log_index);

            self.compute_calls(logs).await?;
        }

        Ok(())
    }
}
