use std::{fs, path::PathBuf};

use crate::utils;

use super::types::{Prover, TargetChain};

use anyhow::Result;
use clap::Args;
use zktls_core::InputBuilder;
use zktls_core::ZkProver;
use zktls_input_builder::TLSInputBuilder;
use zktls_program_core::Request;

#[derive(Args)]
pub struct ProveArgs {
    /// Path to the input request file
    #[arg(short, long)]
    pub input_request_file: PathBuf,

    /// Target chain for the proof
    #[arg(short, long, value_enum)]
    pub target_chain: TargetChain,

    /// Use mock mode
    #[arg(long, group = "proverMode", default_value_t = false)]
    pub mock: bool,

    /// Use local mode
    #[arg(long, group = "proverMode", default_value_t = false)]
    pub local: bool,

    /// Use cuda mode
    #[cfg(feature = "_cuda")]
    #[arg(long, group = "proverMode", default_value_t = false)]
    pub cuda: bool,

    /// Use network mode
    #[arg(long, group = "proverMode", default_value_t = false)]
    pub network: bool,

    /// Prover backend to use
    #[arg(short, long, value_enum)]
    pub prover: Prover,

    /// Moongate server to use
    #[arg(short, long, env = "SP1_MOONGATE_SERVER")]
    pub sp1_moongate_server: Option<String>,
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

        let mut builder = TLSInputBuilder::new().unwrap();
        match builder.build_input(request).await {
            Ok(input) => {
                let output = match self.prover {
                    #[cfg(feature = "r0-backend")]
                    Prover::R0 => {
                        let mut guest = zktls_guest_prover_r0::Risc0GuestProver::default();

                        if self.mock {
                            guest = guest.mock();
                        }

                        if self.local {
                            guest = guest.local();
                        }

                        #[cfg(feature = "r0-backend-cuda")]
                        if self.cuda {
                            guest = guest.cuda();
                        }

                        if self.network {
                            guest = guest.network();
                        }

                        let program = utils::get_program("r0").await?;
                        guest.prove(input, &program).await?
                    }
                    #[cfg(feature = "sp1-backend")]
                    Prover::Sp1 => {
                        let mut guest = zktls_guest_prover_sp1::SP1GuestProver::new(
                            self.sp1_moongate_server.clone(),
                        );
                        if self.mock {
                            guest = guest.mock();
                        }
                        if self.local {
                            guest = guest.local();
                        }
                        #[cfg(feature = "sp1-backend-cuda")]
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

                println!("output: {:?}", output.0);
                println!("proof: {:?}", output.1);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        Ok(())
    }
}
