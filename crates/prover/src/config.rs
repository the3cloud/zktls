use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The duration to sleep between each polling cycle (in seconds)
    pub sleep_duration: u64,

    /// The number of loops to run
    pub loop_number: Option<u64>,
}
