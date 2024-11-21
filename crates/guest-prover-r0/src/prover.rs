use std::panic;

use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use t3zktls_core::{GuestInput, GuestOutput, GuestProver};

pub struct Risc0GuestProver {}

impl GuestProver for Risc0GuestProver {
    async fn prove(&mut self, guest_input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
        panic_catched_prover(guest_input)
    }
}

fn panic_catched_prover(input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    panic::catch_unwind(move || prover(input)).map_err(|e| anyhow::anyhow!("{:?}", e))?
}

fn prover(input: GuestInput) -> Result<(GuestOutput, Vec<u8>)> {
    let prover = default_prover();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prove_result = prover
        .prove_with_opts(env, t3zktls_program_r0::TLS_R0_ELF, &ProverOpts::groth16())
        .unwrap();

    let journal = prove_result.receipt.journal;
    let guest_output: GuestOutput = journal.decode()?;

    let proof = prove_result.receipt.inner.groth16()?.seal.clone();

    Ok((guest_output, proof))
}
