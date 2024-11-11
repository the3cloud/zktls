use alloy::primitives::{Address, B256};

/// Configuration for the ZkTLS Gateway listener
pub struct Config {
    /// The Ethereum address of the ZkTLS Gateway contract
    pub gateway_address: Address,
    /// The block number to start listening from
    pub begin_block_number: u64,
    /// The number of blocks to process in each batch
    pub block_number_batch_size: u64,
    /// The prover id to filter by
    pub prover_id: B256,
}
