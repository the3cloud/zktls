use std::time::Duration;

use anyhow::Result;
use t3zktls_core::{GuestProver, InputBuilder, Listener, ProveResponse, Submiter};

use crate::ProverConfig;

pub struct ZkTLSProver<L, I, G, S> {
    listener: L,
    input_builder: I,
    guest_prover: G,
    submitter: S,

    config: ProverConfig,

    loop_number: u64,
}

impl<L, I, G, S> ZkTLSProver<L, I, G, S> {
    pub fn new(
        config: ProverConfig,
        listener: L,
        input_builder: I,
        guest_prover: G,
        submitter: S,
    ) -> Self {
        Self {
            config,
            listener,
            input_builder,
            guest_prover,
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
            let request = self.listener.pull().await?;

            let request_id = request.request_id;

            let input = self.input_builder.build_input(request).await?;

            let output = self.guest_prover.prove(input).await?;
            self.submitter
                .submit(ProveResponse {
                    request_id,
                    response_data: output.response_data.into(),
                    request_hash: output.request_hash.into(),
                })
                .await?;

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
