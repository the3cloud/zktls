use std::panic;

use alloy::{
    primitives::{Address, Bytes, B256},
    sol_types::SolValue,
};
use anyhow::Result;
use sp1_sdk::{ProverClient, SP1Stdin};
use t3zktls_core::ZkProver;
use t3zktls_program_core::{GuestInput, Response};

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
    async fn prove(&mut self, guest_input: GuestInput, guest_program: &[u8]) -> Result<Response> {
        let is_mock = self.mock;
        let guest_program = guest_program.to_vec();

        tokio::task::spawn_blocking(move || {
            _panic_catched_prove(is_mock, guest_input, guest_program)
        })
        .await?
    }
}

fn _panic_catched_prove(
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
    let mut proof = prover_output.bytes();

    log::debug!("proof: {}", hex::encode(&proof));

    if proof.len() <= 4 {
        proof = Vec::new();
    }

    let (request_id, client, dapp, max_gas_price, max_gas_limit, response) =
        GuestOutputABIType::abi_decode(&output, false)?;

    Ok(Response {
        request_id,
        client,
        dapp,
        max_gas_price,
        max_gas_limit,
        response: response.into(),
        proof,
        prover_id: Default::default(),
    })
}

type GuestOutputABIType = (B256, Address, B256, u64, u64, Bytes);

// #[cfg(test)]
// mod tests {
//     use t3zktls_core::{GuestInput, GuestProver};

//     use super::SP1GuestProver;

//     #[tokio::test]
//     async fn test_prove() {
//         let guest_input_bytes = include_bytes!("../testdata/guest_input0.cbor");

//         let guest_input: GuestInput = ciborium::from_reader(guest_input_bytes.as_slice()).unwrap();

//         let mut prover = SP1GuestProver::default().mock();

//         let (_guest_output, _proof) = prover.prove(guest_input).await.unwrap();
//     }
// }
