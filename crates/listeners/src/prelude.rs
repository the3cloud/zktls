use alloy::primitives::Bytes;
use anyhow::Result;
use std::future::Future;

/// Trait for handling TLS call requests
pub trait HandleRequestTLSCall {
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
        url: &str,
        data: Bytes,
        max_cycle_num: u64,
    ) -> impl Future<Output = Result<()>> + Send;
}

/// Default implementation for HandleRequestTLSCall
impl HandleRequestTLSCall for () {
    async fn handle_request_tls_call(
        &mut self,
        _url: &str,
        _data: Bytes,
        _max_cycle_num: u64,
    ) -> Result<()> {
        let data_str = String::from_utf8(_data.to_vec()).unwrap();

        log::trace!("url: {}, data: {:?}", _url, data_str);

        Ok(())
    }
}

/// Trait for decoding TLS data
pub trait DecodeTLSData {
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
    fn decode_tls_data(
        &self,
        data: &mut Bytes,
        encrypted_key: &Bytes,
    ) -> impl Future<Output = Result<()>> + Send;
}

/// Default implementation for DecodeTLSData
impl DecodeTLSData for () {
    async fn decode_tls_data(&self, _data: &mut Bytes, _encrypted_key: &Bytes) -> Result<()> {
        log::trace!("data: {:?}, encrypted_key: {:?}", _data, _encrypted_key);
        Ok(())
    }
}
