use alloy::primitives::Address;

pub struct Config {
    pub gateway_address: Address,
    pub begin_block_number: u64,
    pub block_number_batch_size: u64,
    pub sleep_duration: u64,
}
