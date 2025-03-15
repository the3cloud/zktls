use std::{future::Future, panic};

use alloy_primitives::{hex, B256};
use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use t3zktls_core::ZkProver;
use zktls_program_core::{GuestInput, Response};

#[derive(Default)]
pub struct Risc0GuestProver {}

impl ZkProver for Risc0GuestProver {
    fn prove(
        &mut self,
        input: GuestInput,
        _pvkey: B256,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<Response>> + Send {
        panic_catched_prover(input, guest_program)
    }
}

async fn panic_catched_prover(input: GuestInput, guest_program: &[u8]) -> Result<Response> {
    panic::catch_unwind(move || prover(input, guest_program))
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
}

fn prover(input: GuestInput, guest_program: &[u8]) -> Result<Response> {
    let prover = default_prover();

    let env = ExecutorEnv::builder().write(&input)?.build()?;

    let prove_result = prover.prove_with_opts(env, guest_program, &ProverOpts::groth16())?;

    let journal = prove_result.receipt.journal;
    let mut response: Response = journal.decode()?;

    let mut proof = prove_result.receipt.inner.groth16()?.seal.clone();

    log::debug!("proof: {}", hex::encode(&proof));

    if proof.len() <= 4 {
        proof = Vec::new();
    }
    response.proof = proof.into();

    Ok(response)
}
