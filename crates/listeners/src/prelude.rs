use alloy::primitives::Bytes;
use anyhow::Result;
use std::future::Future;

pub trait HandleRequestTLSCall {
    fn handle_request_tls_call(
        &mut self,
        url: &str,
        data: Bytes,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl HandleRequestTLSCall for () {
    async fn handle_request_tls_call(&mut self, _url: &str, _data: Bytes) -> Result<()> {
        let data_str = String::from_utf8(_data.to_vec()).unwrap();

        log::trace!("url: {}, data: {:?}", _url, data_str);

        Ok(())
    }
}

pub trait DecodeTLSData {
    fn decode_tls_data(
        &self,
        data: &mut Bytes,
        encrypted_key: &Bytes,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl DecodeTLSData for () {
    async fn decode_tls_data(&self, _data: &mut Bytes, _encrypted_key: &Bytes) -> Result<()> {
        log::trace!("data: {:?}, encrypted_key: {:?}", _data, _encrypted_key);
        Ok(())
    }
}
