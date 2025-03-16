use std::{future::Future, panic};

use alloy_primitives::hex;
use anyhow::Result;
use sp1_sdk::{ProverClient, SP1Stdin};
use t3zktls_core::ZkProver;
use zktls_program_core::{GuestInput, Response};

#[derive(Default)]
pub struct SP1GuestProver {
    mock: bool,
}

impl SP1GuestProver {
    pub fn mock(mut self) -> Self {
        self.mock = true;
        self
    }
}
impl ZkProver for SP1GuestProver {
    fn prove(
        &mut self,
        input: GuestInput,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<Response>> + Send {
        let is_mock = self.mock;
        let guest_program = guest_program.to_vec();

        _panic_catched_prove(is_mock, input, guest_program)
    }
}

async fn _panic_catched_prove(
    is_mock: bool,
    input: GuestInput,
    guest_program: Vec<u8>,
) -> Result<Response> {
    panic::catch_unwind(move || {
        let client = if is_mock {
            ProverClient::mock()
        } else {
            ProverClient::new()
        };

        prove(client, input, &guest_program)
    })
    .map_err(|e| anyhow::anyhow!("{:?}", e))?
}

pub fn prove(client: ProverClient, input: GuestInput, guest_program: &[u8]) -> Result<Response> {
    let mut stdin = SP1Stdin::new();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&input, &mut input_bytes)?;

    stdin.write_vec(input_bytes);

    let (pk, vk) = client.setup(guest_program);

    let prover_output = client.prove(&pk, stdin).groth16().run()?;

    client.verify(&prover_output, &vk)?;

    let output = prover_output.public_values.to_vec();
    let mut response: Response = ciborium::from_reader(output.as_slice())?;

    let mut proof = prover_output.bytes();

    log::debug!("proof: {}", hex::encode(&proof));

    if proof.len() <= 4 {
        proof = Vec::new();
    }
    response.proof = proof.into();

    Ok(response)
}
