/// A listener for ZkTLS Gateway events.
///
/// This struct is responsible for listening to events emitted by the ZkTLS Gateway contract,
/// processing them, and handling TLS call requests.
use std::{collections::HashMap, thread::sleep, time::Duration};

use alloy::{
    network::Network,
    primitives::{Address, Bytes, B256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
    transports::Transport,
};
use anyhow::Result;
use t3_zktls_contracts_ethereum::ZkTLSGateway::{RequestTLSCallBegin, RequestTLSCallSegment};

use crate::{Config, DecodeTLSData, HandleRequestTLSCall};

/// The main Listener struct.
pub struct Listener<P, H, D, T, N> {
    provider: P,
    gateway_address: Address,

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
    H: HandleRequestTLSCall,
    D: DecodeTLSData,
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
    async fn compute_call(&self, logs: &[Log]) -> Result<(String, Bytes, u64)> {
        let mut url = String::new();
        let mut data = Vec::new();
        let mut encrypted_key = Bytes::new();
        let max_cycle_num = 0;

        for log in logs {
            if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallBegin::SIGNATURE_HASH
            {
                // TODO: filter by prover id.

                let decoded = RequestTLSCallBegin::decode_log_data(log.data(), false)?;
                url = decoded.url;
                encrypted_key = decoded.encrypted_key;
            } else if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallSegment::SIGNATURE_HASH
            {
                let decoded = RequestTLSCallSegment::decode_log_data(log.data(), false)?;

                if !decoded.is_encrypted {
                    data.extend_from_slice(&decoded.data);
                } else {
                    let mut dd = decoded.data;

                    self.decryptor
                        .decode_tls_data(&mut dd, &encrypted_key)
                        .await?;
                    data.extend_from_slice(&dd);
                }
            }
        }

        Ok((url, Bytes::from(data), max_cycle_num))
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

            let (url, data, max_cycle_num) = self.compute_call(logs).await?;

            self.handler
                .handle_request_tls_call(&url, data, max_cycle_num)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::{
        network::Ethereum,
        primitives::Address,
        providers::{ReqwestProvider, RootProvider},
        transports::http::reqwest::Url,
    };

    use crate::{Config, Listener};

    fn init_test_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_init_test_logger() {
        init_test_logger();

        // TODO: Use Anvil instance to test in process.

        let config = Config {
            gateway_address: Address::from_str("0x0E801D84Fa97b50751Dbf25036d067dCf18858bF")
                .unwrap(),
            begin_block_number: 0,
            block_number_batch_size: 100,
            sleep_duration: 1,
        };

        let provider: RootProvider<_, Ethereum> =
            ReqwestProvider::new_http(Url::parse("http://localhost:8545").unwrap());

        let mut listener = Listener::new(Some(3), config, provider, (), ());

        listener.pull_blocks().await.unwrap();
    }
}
