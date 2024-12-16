use std::time::Duration;

use alloy::primitives::B256;
use anyhow::Result;
use t3zktls_core::{InputBuilder, RequestGenerator, Submiter, ZkProver};
use tokio::fs;

use crate::Config;

pub struct ZkTLSProver<G, I, P, S> {
    generator: G,
    input_builder: I,
    guest: P,
    submitter: S,

    loop_number: Option<u64>,
    sleep_duration: u64,

    prover_id: B256,

    guest_program: Vec<u8>,
}

impl<G, I, P, S> ZkTLSProver<G, I, P, S> {
    pub async fn new(
        config: Config,
        generator: G,
        input_builder: I,
        guest: P,
        submitter: S,
    ) -> Result<Self> {
        let guest_program = fs::read(&config.guest_program_path).await?;

        Ok(Self {
            generator,
            input_builder,
            guest,
            submitter,

            loop_number: config.loop_number,
            sleep_duration: config.sleep_duration,
            prover_id: config.prover_id,

            guest_program,
        })
    }
}

impl<G, I, P, S> ZkTLSProver<G, I, P, S>
where
    G: RequestGenerator,
    I: InputBuilder,
    P: ZkProver,
    S: Submiter,
{
    pub async fn run(&mut self) -> Result<()> {
        // TODO: Add parallel prove and submiter
        loop {
            let requests = self.generator.generate_requests().await?;

            for request in requests {
                let request_id = request.request_id()?;

                let input = self.input_builder.build_input(request).await;

                if let Ok(input) = input {
                    let mut output = self.guest.prove(input, &self.guest_program).await?;

                    output.prover_id = self.prover_id;

                    let submit_result = self.submitter.submit(output).await;

                    if let Err(e) = submit_result {
                        log::warn!("Submit proof failed: {}, {}", request_id, e);
                    }
                } else {
                    log::warn!("build input failed: {:?}", input.err());
                }
            }
            if let Some(loop_number) = &mut self.loop_number {
                if *loop_number == 0 {
                    break Ok(());
                }

                *loop_number -= 1;
            }

            tokio::time::sleep(Duration::from_secs(self.sleep_duration)).await;
        }
    }
}
