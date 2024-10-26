use std::{collections::HashMap, thread::sleep, time::Duration};

use alloy::{
    primitives::{Address, Bytes, B256},
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
};
use anyhow::Result;
use t3_zktls_contracts_ethereum::ZkTLSGateway::{RequestTLSCallBegin, RequestTLSCallSegment};

use crate::{Config, DecodeTLSData, HandleRequestTLSCall};

pub struct Listener<P, H, D> {
    provider: P,
    gateway_address: Address,

    begin_block_number: u64,
    end_block_number: u64,
    block_number_batch_size: u64,

    sleep_duration: Duration,

    loop_number: Option<u64>,

    handler: H,
    decryptor: D,
}

impl<P: Provider, H: HandleRequestTLSCall, D: DecodeTLSData> Listener<P, H, D> {
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
            end_block_number: config.begin_block_number,
            sleep_duration: Duration::from_secs(config.sleep_duration),
            loop_number,
            handler,
            decryptor,
        }
    }

    async fn pull_block(&mut self) -> Result<()> {
        let latest_block_number = self.provider.get_block_number().await?;
        let filter = self.compute_log_filter(latest_block_number);
        let logs = self.provider.get_logs(&filter).await?;

        self.tidy_logs_and_call(logs).await?;

        if let Some(loop_number) = &mut self.loop_number {
            *loop_number -= 1;
        }

        Ok(())
    }

    pub async fn pull_blocks(&mut self) -> Result<()> {
        while self.loop_number.is_some() {
            self.pull_block().await?;

            sleep(self.sleep_duration);
        }

        loop {
            self.pull_block().await?;

            sleep(self.sleep_duration);
        }
    }

    pub fn compute_log_filter(&mut self, latest_block_number: u64) -> Filter {
        let to_block_number =
            if latest_block_number < self.begin_block_number + self.block_number_batch_size {
                latest_block_number
            } else {
                self.begin_block_number + self.block_number_batch_size
            };

        self.end_block_number = to_block_number;

        let topics = vec![
            RequestTLSCallBegin::SIGNATURE_HASH,
            RequestTLSCallSegment::SIGNATURE_HASH,
        ];

        Filter::new()
            .from_block(self.begin_block_number)
            .to_block(to_block_number)
            .address(self.gateway_address)
            .event_signature(topics)
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
