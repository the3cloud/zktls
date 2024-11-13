use std::time::Duration;

use anyhow::Result;
use t3zktls_core::{GuestProver, InputBuilder, Listener, ProveResponse, Submiter};

use crate::Config;

pub struct ZkTLSProver<L, I, G, S> {
    listener: L,
    input_builder: I,
    guest: G,
    submitter: S,

    config: Config,

    loop_number: u64,
}

impl<L, I, G, S> ZkTLSProver<L, I, G, S> {
    pub fn new(config: Config, listener: L, input_builder: I, guest: G, submitter: S) -> Self {
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

impl<L, I, G, S> ZkTLSProver<L, I, G, S>
where
    L: Listener,
    I: InputBuilder,
    G: GuestProver,
    S: Submiter,
{
    pub async fn run(&mut self) -> Result<()> {
        loop {
            let requests = self.listener.pull().await?;

            for request in requests {
                let request_id = request.request_id;

                let input = self.input_builder.build_input(request).await?;

                let (output, proof) = self.guest.prove(input).await?;

                self.submitter
                    .submit(ProveResponse {
                        request_id,
                        response_data: output.response_data.into(),
                        request_hash: output.request_hash.into(),
                        proof: proof.into(),
                    })
                    .await?;
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
