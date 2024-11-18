use std::num::NonZeroUsize;

use alloy::primitives::B256;
use anyhow::Result;
use t3zktls_core::{
    FilteredResponse, GuestInput, GuestInputRequest, InputBuilder, ProveRequest, ResponseTemplate,
};

use crate::{regex_cache::RegexCache, request::request_tls_call, Config};

pub struct TLSInputBuilder {
    cache: RegexCache,
}

impl TLSInputBuilder {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            cache: RegexCache::new(
                NonZeroUsize::new(config.regex_cache_size)
                    .ok_or(anyhow::anyhow!("regex_cache_size must be greater than 0"))?,
            ),
        })
    }
}

impl InputBuilder for TLSInputBuilder {
    async fn build_input(&mut self, request: ProveRequest) -> Result<GuestInput> {
        self.handle_request_tls_call(request).await
    }
}

impl TLSInputBuilder {
    async fn handle_request_tls_call(&mut self, req: ProveRequest) -> Result<GuestInput> {
        let guest_input_request = GuestInputRequest {
            url: req.remote,
            server_name: req.server_name,
            request: req.request,
            encrypted_key: req.encrypted_key,
        };

        let guest_input_request_cloned = guest_input_request.clone();

        let mut guest_input_response =
            tokio::task::spawn_blocking(move || request_tls_call(guest_input_request_cloned))
                .await??;

        let response = guest_input_response.response.clone();

        let mut filtered_responses = Vec::new();

        match req.response_template {
            ResponseTemplate::None => {}
            ResponseTemplate::Position { begin, length } => {
                let fr = self.handle_response_template_position(begin, length, &response)?;

                filtered_responses.push(fr);
            }
            ResponseTemplate::Regex(regex) => {
                let fr = self.handle_response_template_regex(
                    req.response_template_id,
                    regex,
                    String::from_utf8(response.clone())?,
                )?;

                filtered_responses.extend_from_slice(fr.as_slice());
            }
            ResponseTemplate::XPath(xpath) => {
                let fr = self
                    .handle_response_template_xpath(xpath, String::from_utf8(response.clone())?)?;

                filtered_responses.extend_from_slice(fr.as_slice());
            }
            ResponseTemplate::JsonPath(json_path) => {
                let fr = self.handle_response_template_jsonpath(
                    json_path,
                    String::from_utf8(response.clone())?,
                )?;

                filtered_responses.extend_from_slice(fr.as_slice());
            }
        }

        guest_input_response.filtered_responses = filtered_responses;

        let guest_input = GuestInput {
            request: guest_input_request,
            response: guest_input_response,
        };

        Ok(guest_input)
    }

    fn handle_response_template_position(
        &mut self,
        begin: u64,
        length: u64,
        s: &[u8],
    ) -> Result<FilteredResponse> {
        let content = s[begin as usize..(begin + length) as usize].to_vec();

        Ok(FilteredResponse {
            begin,
            length,
            content,
        })
    }

    fn handle_response_template_regex(
        &mut self,
        template_id: B256,
        regex: String,
        response: String,
    ) -> Result<Vec<FilteredResponse>> {
        let filtered_responses = self.cache.find(template_id, &regex, &response)?;

        Ok(filtered_responses)
    }

    fn handle_response_template_xpath(
        &mut self,
        _xpath: String,
        _response: String,
    ) -> Result<Vec<FilteredResponse>> {
        Err(anyhow::anyhow!("not implemented"))
    }

    fn handle_response_template_jsonpath(
        &mut self,
        _json_path: String,
        _response: String,
    ) -> Result<Vec<FilteredResponse>> {
        Err(anyhow::anyhow!("not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use t3zktls_core::ProveRequest;

    use crate::{Config, TLSInputBuilder};

    #[tokio::test]
    async fn test_handle_response1() {
        let bytes = include_bytes!("../testdata/req0.cbor");

        let req: ProveRequest = ciborium::from_reader(bytes.as_slice()).unwrap();

        let config = Config {
            regex_cache_size: 100,
        };

        let mut builder = TLSInputBuilder::new(config).unwrap();

        let input = builder.handle_request_tls_call(req).await.unwrap();

        println!(
            "response: {}",
            String::from_utf8(input.response.response.clone()).unwrap()
        );

        let mut guest_input_bytes = Vec::new();
        ciborium::ser::into_writer(&input, &mut guest_input_bytes).unwrap();

        fs::write("../../target/guest_input0.cbor", guest_input_bytes).unwrap();
    }
}
