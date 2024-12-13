use std::future::Future;

use anyhow::Result;
use t3zktls_program_core::{GuestInput, Request, Response};

pub trait RequestGenerator {
    fn generate_request(&mut self) -> impl Future<Output = Result<Request>> + Send;
}

pub trait InputBuilder {
    fn build_input(&mut self, request: Request) -> impl Future<Output = Result<GuestInput>> + Send;
}

pub trait ZkProver {
    fn prove(&mut self, input: GuestInput) -> impl Future<Output = Result<Response>> + Send;
}

pub trait Submiter {
    fn submit(&mut self, response: Response) -> impl Future<Output = Result<()>> + Send;
}
