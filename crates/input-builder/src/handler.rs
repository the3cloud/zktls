use std::num::NonZeroUsize;

use anyhow::Result;
use zktls_core::InputBuilder;
use zktls_program_core::{GuestInput, Request, ResponseTemplate};

use crate::{regex_cache::RegexCache, request::request_tls_call, Config, FilteredResponse};

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
    async fn build_input(&mut self, request: Request) -> Result<GuestInput> {
        self.handle_request_tls_call(request).await
    }
}

impl TLSInputBuilder {
    async fn handle_request_tls_call(&mut self, req: Request) -> Result<GuestInput> {
        // OPT: avoid cloning
        let req_cloned = req.clone();

        let mut guest_input_response =
            tokio::task::spawn_blocking(move || request_tls_call(&req_cloned)).await??;

        for template in &req.response_template {
            match template {
                ResponseTemplate::Offset { begin, length } => {
                    let fr = self.handle_response_template_position(
                        *begin,
                        *length,
                        &guest_input_response.response,
                    )?;

                    guest_input_response.filtered_responses_begin.push(fr.begin);
                    guest_input_response
                        .filtered_responses_length
                        .push(fr.length);
                    guest_input_response
                        .filtered_responses
                        .push(fr.bytes.into());
                }
                ResponseTemplate::Regex { pattern } => {
                    let fr = self.handle_response_template_regex(
                        pattern.as_str(),
                        &guest_input_response.response,
                    )?;

                    guest_input_response
                        .filtered_responses_begin
                        .extend(fr.iter().map(|fr| fr.begin));
                    guest_input_response
                        .filtered_responses_length
                        .extend(fr.iter().map(|fr| fr.length));
                    guest_input_response
                        .filtered_responses
                        .extend(fr.into_iter().map(|fr| fr.bytes.into()));
                }
            }
        }

        Ok(GuestInput {
            request: req,
            response: guest_input_response,
        })
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
            bytes: content,
        })
    }

    fn handle_response_template_regex(
        &mut self,
        regex: &str,
        response: &[u8],
    ) -> Result<Vec<FilteredResponse>> {
        let filtered_responses = self
            .cache
            .find(regex, &String::from_utf8(response.to_vec())?)?;

        Ok(filtered_responses)
    }

    // fn handle_response_template_xpath(
    //     &mut self,
    //     _xpath: String,
    //     _response: String,
    // ) -> Result<Vec<FilteredResponse>> {
    //     Err(anyhow::anyhow!("not implemented"))
    // }

    // fn handle_response_template_jsonpath(
    //     &mut self,
    //     _json_path: String,
    //     _response: String,
    // ) -> Result<Vec<FilteredResponse>> {
    //     Err(anyhow::anyhow!("not implemented"))
    // }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use zktls_program_core::Request;

    use crate::{Config, TLSInputBuilder};

    #[tokio::test]
    async fn test_handle_response1() {
        let bytes = include_str!("../testdata/req0.json");

        let mut req: Request = serde_json::from_str(bytes).unwrap();

        let request_body =
            "GET /get HTTP/1.1\r\nHost: httpbin.org\r\nAccept: */*\r\nConnection: Close\r\n\r\n";

        req.request_info.request = request_body.as_bytes().to_vec().into();

        let config = Config {
            regex_cache_size: 100,
        };

        let mut builder = TLSInputBuilder::new(config).unwrap();

        let input = builder.handle_request_tls_call(req).await.unwrap();

        println!(
            "response: {}",
            String::from_utf8(input.response.response.clone()).unwrap()
        );

        let guest_input_bytes = serde_json::to_string_pretty(&input).unwrap();

        fs::write("../../target/guest_input0.json", guest_input_bytes).unwrap();
    }
}
