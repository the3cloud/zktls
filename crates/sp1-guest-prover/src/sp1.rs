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
        let client = if self.mock {
            ProverClient::mock()
        } else {
            ProverClient::new()
        };

        tokio::task::spawn_blocking(move || prove(&client, guest_input)).await?
    }
}

pub fn prove(client: &ProverClient, input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    sp1_sdk::utils::setup_logger();

    // let client = ProverClient::new();
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
