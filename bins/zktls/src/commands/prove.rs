use std::{fs, path::PathBuf};

use crate::utils;

use super::types::{Prover, TargetChain};

use anyhow::Result;
use clap::Args;
use zktls_core::InputBuilder;
use zktls_core::ZkProver;
use zktls_input_builder::{Config, TLSInputBuilder};
use zktls_program_core::Request;

#[derive(Args)]
pub struct ProveArgs {
    /// Path to the input request file
    #[arg(short, long)]
    pub input_request_file: PathBuf,

    /// Target chain for the proof
    #[arg(long, value_enum)]
    pub target_chain: TargetChain,

    /// Use mock mode
    #[arg(long, group = "proverMode")]
    pub mock: bool,

    /// Use local mode
    #[arg(long, group = "proverMode")]
    pub local: bool,

    /// Use cuda mode
    #[cfg(feature = "cuda")]
    #[arg(long, group = "proverMode")]
    pub cuda: bool,

    #[arg(long, group = "proverMode")]
    pub network: bool,

    /// Prover backend to use
    #[arg(long, value_enum)]
    pub prover: Prover,
}

impl ProveArgs {
    pub async fn execute(&self) -> Result<()> {
        // Validate that input file exists
        if !self.input_request_file.exists() {
            return Err(anyhow::anyhow!(
                "Input request file does not exist: {}",
                self.input_request_file.display()
            ));
        }

        let input_request_file = fs::read_to_string(&self.input_request_file)?;
        let request: Request = serde_json::from_str(&input_request_file)?;
        let config = Config {
            regex_cache_size: 100,
        };

        let mut builder = TLSInputBuilder::new(config).unwrap();
        match builder.build_input(request).await {
            Ok(input) => {
                let output = match self.prover {
                    #[cfg(feature = "r0-backend")]
                    Prover::R0 => {
                        let mut guest = zktls_guest_prover_r0::Risc0GuestProver::default();
                        let program = utils::get_program("r0").await?;
                        guest.prove(input, &program).await?
                    }
                    #[cfg(feature = "sp1-backend")]
                    Prover::Sp1 => {
                        let mut guest = zktls_guest_prover_sp1::SP1GuestProver::default();
                        if self.mock {
                            guest = guest.mock();
                        }
                        if self.local {
                            guest = guest.local();
                        }
                        #[cfg(feature = "cuda")]
                        if self.cuda {
                            guest = guest.cuda();
                        }
                        if self.network {
                            guest = guest.network();
                        }
                        let program = utils::get_program("sp1").await?;
                        guest.prove(input, &program).await?
                    }
                };
                println!(
                    "Submiting output for request id: {}, client is: {}, dapp hash is: {}, with max gas price: {} and max gas limit: {}",
                    output.request_id,
                    output.client,
                    output.dapp,
                    output.max_gas_price,
                    output.max_gas_limit
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        Ok(())
    }
}
