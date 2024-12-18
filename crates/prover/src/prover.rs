use alloy::primitives::B256;
use anyhow::Result;
use t3zktls_core::{InputBuilder, RequestGenerator, Submiter, ZkProver};
use tokio::fs;

use crate::Config;

pub struct ZkTLSProver<G, I, P, S> {
    generator: G,
    input_builder: I,
    guest: P,
    submitter: Option<S>,

    prover_id: B256,

    guest_program: Vec<u8>,

    pvkey: B256,
}

impl<G, I, P, S> ZkTLSProver<G, I, P, S> {
    pub async fn new(
        config: Config,
        generator: G,
        input_builder: I,
        guest: P,
        submitter: Option<S>,
    ) -> Result<Self> {
        let guest_program = fs::read(&config.guest_program_path).await?;

        Ok(Self {
            generator,
            input_builder,
            guest,
            submitter,

            prover_id: config.prover_id,

            guest_program,
            pvkey: config.pvkey,
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
        let requests = self.generator.generate_requests().await?;

        for request in requests {
            let request_id = request.request_id()?;

            log::info!("request id: {}", request_id);

            let input = self.input_builder.build_input(request).await;

            if let Ok(input) = input {
                let mut output = self
                    .guest
                    .prove(input, self.pvkey.clone(), &self.guest_program)
                    .await?;

                output.prover_id = self.prover_id;

                log::info!(
                    "Submiting output for request id: {}, client is: {}, dapp hash is: {}, with max gas price: {} and max gas limit: {}",
                    output.request_id,
                    output.client,
                    output.dapp,
                    output.max_gas_price,
                    output.max_gas_limit
                );

                if let Some(submitter) = &mut self.submitter {
                    let submit_result = submitter.submit(output).await;

                    if let Err(e) = submit_result {
                        log::warn!("Submit proof failed: {}, {}", request_id, e);
                    }
                }
            } else {
                log::warn!("build input failed: {:?}", input.err());
            }
        }

        Ok(())
    }
}
