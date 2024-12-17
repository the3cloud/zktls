use std::path::PathBuf;

use alloy::primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// The duration to sleep between each polling cycle (in seconds)
    pub sleep_duration: u64,

    /// The number of loops to run
    pub loop_number: Option<u64>,

    pub guest_program_path: PathBuf,

    pub prover_id: B256,

    pub pvkey: B256,
}
