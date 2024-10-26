use alloy::primitives::Bytes;
use anyhow::Result;
use std::future::Future;

pub trait HandleRequestTLSCall {
    fn handle_request_tls_call(
        &mut self,
        url: &str,
        data: &[Bytes],
    ) -> impl Future<Output = Result<()>> + Send;
}

pub trait DecodeTLSData {
    fn decode_tls_data(
        &self,
        data: &mut Bytes,
        encrypted_key: &Bytes,
    ) -> impl Future<Output = Result<()>> + Send;
}
