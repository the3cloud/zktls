use std::{fs, path::PathBuf};

use crate::utils::get_r0_program;
use crate::utils::get_sp1_program;

use super::types::{Prover, TargetChain};

use anyhow::Result;
use clap::Args;
use t3zktls_core::InputBuilder;
use t3zktls_core::ZkProver;
use t3zktls_guest_prover_r0::Risc0GuestProver;
use t3zktls_guest_prover_sp1::SP1GuestProver;
use t3zktls_input_builder::{Config, TLSInputBuilder};
use zktls_program_core::Request;

#[derive(Args)]
pub struct ProveArgs {
    /// Path to the input request file
    #[arg(long)]
    pub input_request_file: PathBuf,

    /// Target chain for the proof
    #[arg(long, value_enum)]
    pub target_chain: TargetChain,

    /// Use mock mode
    #[arg(long)]
    pub mock: bool,

    /// Prover backend to use
    #[arg(long, value_enum)]
    pub prover: Prover,
}

impl ProveArgs {
    pub async fn execute(&self) -> Result<()> {
        // Validate that input file exists
        if !self.input_request_file.exists() {
            anyhow::bail!(
                "Input request file does not exist: {}",
                self.input_request_file.display()
            );
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
                    Prover::R0 => {
                        let mut guest = Risc0GuestProver::default();
                        let program = get_r0_program().await?;
                        guest.prove(input, &program).await?
                    }
                    Prover::Sp1 => {
                        let mut guest = SP1GuestProver::default();
                        if self.mock {
                            guest = guest.mock();
                        }
                        let program = get_sp1_program().await?;
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
