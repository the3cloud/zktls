use alloy::primitives::Bytes;
use anyhow::Result;
use std::future::Future;

use crate::{GuestInput, GuestOutput, ProveRequest};

/// Trait for handling TLS call requests
pub trait RequestTLSCallHandler {
    /// Handles a TLS call request
    ///
    /// # Arguments
    ///
    /// * `url` - The URL for the TLS call
    /// * `data` - The data for the TLS call
    /// * `max_cycle_num` - The maximum number of zkVM cycles for the call
    ///
    /// # Returns
    ///
    /// A future that resolves to a Result
    fn handle_request_tls_call(
        &mut self,
        request: ProveRequest,
    ) -> impl Future<Output = Result<()>> + Send;
}

/// Default implementation for RequestTLSCallHandler
impl RequestTLSCallHandler for () {
    async fn handle_request_tls_call(&mut self, request: ProveRequest) -> Result<()> {
        let data_str = String::from_utf8(request.request_data.to_vec()).unwrap();

        log::trace!(
            "request_id: {}, url: {}, server_name: {}, data: {:?}, response_length: {}",
            request.request_id,
            request.remote.url,
            request.remote.server_name,
            data_str,
            request.max_response_size
        );

        Ok(())
    }
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

pub trait GuestProver {
    fn prove(
        &mut self,
        guest_input: GuestInput,
    ) -> impl Future<Output = Result<GuestOutput>> + Send;
}
