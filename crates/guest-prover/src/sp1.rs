use anyhow::Result;
use sp1_sdk::{ProverClient, SP1Stdin};
use t3zktls_core::{GuestInput, GuestOutput, GuestProver};

#[derive(Default)]
pub struct SP1GuestProver {}

impl GuestProver for SP1GuestProver {
    async fn prove(&mut self, guest_input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
        tokio::task::spawn_blocking(move || prove(guest_input)).await?
    }
}

pub fn prove(input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    sp1_sdk::utils::setup_logger();

    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&input, &mut input_bytes)?;

    stdin.write_vec(input_bytes);

    let (pk, vk) = client.setup(t3zktls_program::TLS_ELF);

    let prover_output = client.prove(&pk, stdin).groth16().run()?;

    client.verify(&prover_output, &vk)?;

    let output = prover_output.public_values.to_vec();
    let proof = prover_output.bytes();

    let guest_output: GuestOutput = ciborium::from_reader(&mut output.as_slice())?;

    Ok((guest_output, proof))
}
