use std::{future::Future, panic};

use alloy_primitives::hex;
use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use zktls_core::ZkProver;
use zktls_program_core::GuestInput;

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
            ProverType::Mock => std::env::set_var("RISC0_DEV_MODE", "true"),
            ProverType::Local => std::env::set_var("RISC0_PROVER", "local"),
            #[cfg(feature = "cuda")]
            ProverType::Cuda => std::env::set_var("RISC0_PROVER", "local"),
            ProverType::Network => std::env::set_var("RISC0_PROVER", "bonsai"),
        }
    }
}

#[derive(Default)]
pub struct Risc0GuestProver {
    mode: ProverType,
}

impl Risc0GuestProver {
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

impl ZkProver for Risc0GuestProver {
    fn prove(
        &mut self,
        input: GuestInput,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<(Vec<u8>, Vec<u8>)>> + Send {
        self.mode.set_env();
        panic_catched_prover(input, guest_program)
    }
}

async fn panic_catched_prover(
    input: GuestInput,
    guest_program: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
    panic::catch_unwind(move || prover(input, guest_program))
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
}

fn prover(input: GuestInput, guest_program: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let prover = default_prover();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&input, &mut input_bytes)?;

    println!("input_len: {:?}", input_bytes.len());

    let env = ExecutorEnv::builder().write_slice(&input_bytes).build()?;

    let start = std::time::Instant::now();

    let prove_result = prover.prove_with_opts(env, guest_program, &ProverOpts::groth16())?;

    let elapsed = start.elapsed();
    println!("Proving took: {:?}", elapsed);

    let journal = prove_result.receipt.journal.bytes;
    let mut proof = prove_result.receipt.inner.groth16()?.seal.clone();

    log::info!("output: {}", hex::encode(&journal));
    log::info!("proof: {}", hex::encode(&proof));

    if proof.len() <= 4 {
        proof = Vec::new();
    }

    Ok((journal, proof))
}
