use std::future::Future;

use alloy::primitives::B256;
use anyhow::Result;
use t3zktls_program_core::{GuestInput, Request, Response};

/// Generate a request from the listener.
pub trait RequestGenerator {
    fn generate_requests(&mut self) -> impl Future<Output = Result<Vec<Request>>> + Send;
}

/// Build the input for the zktls program.
pub trait InputBuilder {
    fn build_input(&mut self, request: Request) -> impl Future<Output = Result<GuestInput>> + Send;
}

/// Prove the request using the zk prover.
pub trait ZkProver {
    fn prove(
        &mut self,
        input: GuestInput,
        pvkey: B256,
        guest_program: &[u8],
    ) -> impl Future<Output = Result<Response>> + Send;
}

/// Submit the response to the chain.
pub trait Submiter {
    fn submit(&mut self, response: Response) -> impl Future<Output = Result<()>> + Send;
}
