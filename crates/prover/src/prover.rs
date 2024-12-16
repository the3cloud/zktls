use std::time::Duration;

use anyhow::Result;
use t3zktls_core::{InputBuilder, RequestGenerator, Submiter, ZkProver};
use tokio::fs;

use crate::Config;

pub struct ZkTLSProver<L, I, G, S> {
    listener: L,
    input_builder: I,
    guest: G,
    submitter: S,

    loop_number: Option<u64>,
    sleep_duration: u64,

    guest_program: Vec<u8>,
}

impl<R, I, G, S> ZkTLSProver<R, I, G, S> {
    pub async fn new(
        config: Config,
        listener: R,
        input_builder: I,
        guest: G,
        submitter: S,
    ) -> Result<Self> {
        let guest_program = fs::read(&config.guest_program_path).await?;

        Ok(Self {
            listener,
            input_builder,
            guest,
            submitter,

            loop_number: config.loop_number,
            sleep_duration: config.sleep_duration,
            guest_program,
        })
    }
}

impl<R, I, G, S> ZkTLSProver<R, I, G, S>
where
    R: RequestGenerator,
    I: InputBuilder,
    G: ZkProver,
    S: Submiter,
{
    pub async fn run(&mut self) -> Result<()> {
        // TODO: Add parallel prove and submiter
        loop {
            let requests = self.listener.generate_requests().await?;

            for request in requests {
                let request_id = request.request_id()?;

                let input = self.input_builder.build_input(request).await;

                if let Ok(input) = input {
                    let output = self.guest.prove(input, &self.guest_program).await?;

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
