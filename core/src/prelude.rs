use std::future::Future;

use anyhow::Result;
use zktls_program_core::{GuestInput, Request};

/// Build the input for the zktls program.
pub trait InputBuilder {
    fn build_input(&mut self, request: Request) -> impl Future<Output = Result<GuestInput>> + Send;
}

/// Prove the request using the zk prover.
pub trait ZkProver {
    fn prove(
        &mut self,
        input: GuestInput,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<(Vec<u8>, Vec<u8>)>> + Send;
}
