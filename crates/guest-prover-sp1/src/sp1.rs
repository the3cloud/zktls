use std::{future::Future, panic};

use alloy_primitives::hex;
use anyhow::Result;
use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{Prover, ProverClient, SP1Stdin};
use zktls_core::ZkProver;
use zktls_program_core::{GuestInput, Response};

#[derive(Default)]
pub enum ProverType {
    #[default]
    Mock,
    Local,
    #[cfg(feature = "cuda")]
    Cuda,
    Network,
}

impl ProverType {
    pub fn set_env(&self) {
        match self {
            ProverType::Mock => std::env::set_var("SP1_PROVER", "mock"),
            ProverType::Local => std::env::set_var("SP1_PROVER", "local"),
            #[cfg(feature = "cuda")]
            ProverType::Cuda => std::env::set_var("SP1_PROVER", "cuda"),
            ProverType::Network => std::env::set_var("SP1_PROVER", "network"),
        }
    }
}

pub struct SP1GuestProver {
    mode: ProverType,
    moongate_server: Option<String>,
}

impl SP1GuestProver {
    pub fn new(moongate_server: Option<String>) -> Self {
        Self {
            mode: ProverType::default(),
            moongate_server,
        }
    }

    pub fn mock(mut self) -> Self {
        self.mode = ProverType::Mock;
        self
    }

    pub fn local(mut self) -> Self {
        self.mode = ProverType::Local;
        self
    }

    #[cfg(feature = "cuda")]
    pub fn cuda(mut self) -> Self {
        self.mode = ProverType::Cuda;
        self
    }

    pub fn network(mut self) -> Self {
        self.mode = ProverType::Network;
        self
    }
}
impl ZkProver for SP1GuestProver {
    fn prove(
        &mut self,
        input: GuestInput,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<Response>> + Send {
        self.mode.set_env();

        let guest_program = guest_program.to_vec();

        _panic_catched_prove(input, guest_program, &self.moongate_server)
    }
}

async fn _panic_catched_prove(
    input: GuestInput,
    guest_program: Vec<u8>,
    moongate_server: &Option<String>,
) -> Result<Response> {
    panic::catch_unwind(move || {
        if let Some(server) = moongate_server {
            let prover = ProverClient::builder()
                .cuda()
                .with_moongate_endpoint(server)
                .build();

            prove(prover, input, &guest_program)
        } else {
            let client = ProverClient::from_env();

            prove(client, input, &guest_program)
        }
    })
    .map_err(|e| anyhow::anyhow!("{:?}", e))?
}

pub fn prove<P>(client: P, input: GuestInput, guest_program: &[u8]) -> Result<Response>
where
    P: Prover<CpuProverComponents>,
{
    let mut stdin = SP1Stdin::new();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&input, &mut input_bytes)?;

    stdin.write_vec(input_bytes);

    let (pk, vk) = client.setup(guest_program);

    let start = std::time::Instant::now();
    let prover_output = client.prove(&pk, &stdin, sp1_sdk::SP1ProofMode::Groth16)?;
    let elapsed = start.elapsed();
    log::info!("Proving time: {:?}", elapsed);

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
