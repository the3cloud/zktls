use std::panic;

use alloy::{
    primitives::{Bytes, B256},
    sol_types::SolValue,
};
use anyhow::Result;
use sp1_sdk::{ProverClient, SP1Stdin};
use t3zktls_core::{GuestInput, GuestOutput, GuestProver};

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

impl GuestProver for SP1GuestProver {
    async fn prove(&mut self, guest_input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
        let is_mock = self.mock;

        tokio::task::spawn_blocking(move || _panic_catched_prove(is_mock, guest_input)).await?
    }
}

fn _panic_catched_prove(is_mock: bool, input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    panic::catch_unwind(move || {
        let client = if is_mock {
            ProverClient::mock()
        } else {
            ProverClient::new()
        };

        prove(client, input)
    })
    .map_err(|e| anyhow::anyhow!("{:?}", e))?
}

pub fn prove(client: ProverClient, input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    let mut stdin = SP1Stdin::new();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&input, &mut input_bytes)?;

    stdin.write_vec(input_bytes);

    let (pk, vk) = client.setup(t3zktls_program_sp1::TLS_ELF);

    let prover_output = client.prove(&pk, stdin).groth16().run()?;

    client.verify(&prover_output, &vk)?;

    let output = prover_output.public_values.to_vec();
    let mut proof = prover_output.bytes();

    if proof.len() <= 4 {
        proof = Vec::new();
    }

    let (request_hash, response_data) = GuestOutputABIType::abi_decode(&output, false)?;

    Ok((
        GuestOutput {
            request_hash: request_hash.0,
            response_data: response_data.into(),
        },
        proof,
    ))
}

type GuestOutputABIType = (B256, Bytes);

#[cfg(test)]
mod tests {
    use t3zktls_core::{GuestInput, GuestProver};

    use super::SP1GuestProver;

    #[tokio::test]
    async fn test_prove() {
        let guest_input_bytes = include_bytes!("../testdata/guest_input0.cbor");

        let guest_input: GuestInput = ciborium::from_reader(guest_input_bytes.as_slice()).unwrap();

        let mut prover = SP1GuestProver::default().mock();

        let (_guest_output, _proof) = prover.prove(guest_input).await.unwrap();
    }
}
