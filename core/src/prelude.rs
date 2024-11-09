use alloy::primitives::Bytes;
use anyhow::Result;
use std::future::Future;

use crate::{GuestInput, GuestOutput, ProveRequest, ProveResponse};

pub trait Listener {
    fn pull(&mut self) -> impl Future<Output = Result<ProveRequest>> + Send;
}

pub trait InputBuilder {
    fn build_input(
        &mut self,
        request: ProveRequest,
    ) -> impl Future<Output = Result<GuestInput>> + Send;
}

pub trait GuestProver {
    fn prove(
        &mut self,
        guest_input: GuestInput,
    ) -> impl Future<Output = Result<GuestOutput>> + Send;
}

pub trait Submiter {
    fn submit(&mut self, prove_response: ProveResponse) -> impl Future<Output = Result<()>> + Send;
}

pub trait TLSDataDecryptorGenerator {
    type Decryptor: TLSDataDecryptor;

    fn generate_decryptor(
        &self,
        encrypted_public_key: &Bytes,
    ) -> impl Future<Output = Result<Self::Decryptor>> + Send;
}

/// Trait for decrypting TLS data
pub trait TLSDataDecryptor {
    /// Decodes TLS data
    ///
    /// # Arguments
    ///
    /// * `data` - The data to decode
    /// * `encrypted_key` - The encrypted key for decoding
    ///
    /// # Returns
    ///
    /// A future that resolves to a Result
    fn decrypt_tls_data(&mut self, data: &mut Vec<u8>) -> impl Future<Output = Result<()>> + Send;
}
