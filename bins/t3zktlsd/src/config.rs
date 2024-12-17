use alloy::primitives::{Address, B256};
use serde::{Deserialize, Serialize};
use t3zktls_submiter_ethereum::Signer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub input_builder: t3zktls_input_builder::Config,
    pub submiter: SubmiterConfig,
    pub prover: t3zktls_prover::Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmiterConfig {
    pub confirmations: u64,
    pub gateway_address: Address,
    pub rpc_url: String,
}

impl SubmiterConfig {
    pub fn build_local_config(self, private_key: B256) -> t3zktls_submiter_ethereum::Config {
        t3zktls_submiter_ethereum::Config {
            confirmations: self.confirmations,
            gateway_address: self.gateway_address,
            rpc_url: self.rpc_url,
            signer: Signer::Local { private_key },
        }
    }
}
