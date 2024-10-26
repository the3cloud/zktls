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
    async fn pull_block(&mut self) -> Result<()> {
        let latest_block_number = self.provider.get_block_number().await?;
        let filter = self.compute_log_filter(latest_block_number);

        if let Some(filter) = filter {
            let logs = self.provider.get_logs(&filter).await?;

            self.tidy_logs_and_call(logs).await?;
        }
        Ok(())
    }

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

    pub fn compute_log_filter(&mut self, latest_block_number: u64) -> Option<Filter> {
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

    async fn compute_call(&self, logs: &[Log]) -> Result<(String, Vec<Bytes>)> {
        let mut url = String::new();
        let mut data = Vec::new();
        for log in logs {
            if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallBegin::SIGNATURE_HASH
            {
                let decoded = RequestTLSCallBegin::decode_log_data(log.data(), false)?;
                url = decoded.url;
            } else if log.topic0().ok_or(anyhow::anyhow!("log topic is empty"))?
                == &RequestTLSCallSegment::SIGNATURE_HASH
            {
                let decoded = RequestTLSCallSegment::decode_log_data(log.data(), false)?;

                if decoded.encrypted_key.is_empty() {
                    data.push(decoded.data);
                } else {
                    let mut dd = decoded.data;

                    self.decryptor
                        .decode_tls_data(&mut dd, &decoded.encrypted_key)
                        .await?;

                    data.push(dd);
                }
            }
        }

        Ok((url, data))
    }

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

            let (url, data) = self.compute_call(logs).await?;

            self.handler.handle_request_tls_call(&url, &data).await?;
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

        let config = Config {
            gateway_address: Address::from_str("0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1")
                .unwrap(),
            begin_block_number: 0,
            block_number_batch_size: 3,
            sleep_duration: 1,
        };

        let provider: RootProvider<_, Ethereum> =
            ReqwestProvider::new_http(Url::parse("http://localhost:8545").unwrap());

        let mut listener = Listener::new(Some(10), config, provider, (), ());

        listener.pull_blocks().await.unwrap();
    }
}
