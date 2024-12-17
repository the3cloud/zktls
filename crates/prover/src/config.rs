use std::path::PathBuf;

use alloy::primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub guest_program_path: PathBuf,

    pub prover_id: B256,

    #[serde(skip)]
    pub pvkey: B256,
}
