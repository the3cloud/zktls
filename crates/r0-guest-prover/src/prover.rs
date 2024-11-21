use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use t3zktls_core::{GuestInput, GuestOutput};

pub struct Risc0GuestProver {}

fn prover(input: GuestInput) -> Result<GuestOutput> {
    let prover = default_prover();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prove_result = prover
        .prove_with_opts(env, t3zktls_program_r0
            
            ::TLS_R0_ELF, &ProverOpts::groth16())
        .unwrap();

    let journal = prove_result.receipt.journal;
    let guest_output: GuestOutput = journal.decode()?;

    Ok(guest_output)
}
