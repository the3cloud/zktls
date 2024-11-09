use anyhow::Result;
use t3zktls_core::{GuestProver, InputBuilder, Listener, ProveResponse, Submiter};

pub struct ZkTLSProver<L, I, G, S> {
    listener: L,
    input_builder: I,
    guest_prover: G,
    submitter: S,
}

impl<L, I, G, S> ZkTLSProver<L, I, G, S> {
    pub fn new(listener: L, input_builder: I, guest_prover: G, submitter: S) -> Self {
        Self {
            listener,
            input_builder,
            guest_prover,
            submitter,
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
        }
    }
}
