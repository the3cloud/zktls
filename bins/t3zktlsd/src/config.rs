use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub listener: t3zktls_listeners_ethereum::Config,
    pub input_builder: t3zktls_input_builder::Config,
    pub submiter: t3zktls_submiter_ethereum::Config,
    pub prover: ProverConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverConfig {
    #[serde(flatten)]
    pub prover: t3zktls_prover::Config,
    pub rpc_url: String,
}
