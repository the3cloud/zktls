use std::time::Duration;

use anyhow::Result;
use t3zktls_core::{InputBuilder, RequestGenerator, Submiter, ZkProver};

use crate::Config;

pub struct ZkTLSProver<L, I, G, S> {
    listener: L,
    input_builder: I,
    guest: G,
    submitter: S,

    config: Config,

    loop_number: u64,
}

impl<R, I, G, S> ZkTLSProver<R, I, G, S> {
    pub fn new(config: Config, listener: R, input_builder: I, guest: G, submitter: S) -> Self {
        Self {
            config,
            listener,
            input_builder,
            guest,
            submitter,

            loop_number: 0,
        }
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
                    let output = self.guest.prove(input).await?;

                    let submit_result = self.submitter.submit(output).await;

                    if let Err(e) = submit_result {
                        log::warn!("Submit proof failed: {}, {}", request_id, e);
                    }
                } else {
                    log::warn!("build input failed: {:?}", input.err());
                }
            }
            if let Some(loop_number) = self.config.loop_number {
                if self.loop_number >= loop_number {
                    break Ok(());
                }
            }
            self.loop_number += 1;

            tokio::time::sleep(Duration::from_secs(self.config.sleep_duration)).await;
        }
    }
}
