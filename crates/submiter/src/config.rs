use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub confirmations: u64,
    pub gateway_address: Address,
}
