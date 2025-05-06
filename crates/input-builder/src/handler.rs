use anyhow::Result;
use memchr::memmem::Finder;
use zktls_core::InputBuilder;
use zktls_program_core::{GuestInput, Request, ResponseTemplate};

use crate::{request::request_tls_call, FilteredResponse};

pub struct TLSInputBuilder {}

impl TLSInputBuilder {
    pub fn new() -> Result<Self> {
        Ok(Self {})
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
                ResponseTemplate::Prefix { prefix, length } => {
                    let fr = self.handle_response_template_prefix(
                        prefix,
                        *length,
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

    fn handle_response_template_prefix(
        &mut self,
        prefix: &[u8],
        length: u64,
        response: &[u8],
    ) -> Result<Vec<FilteredResponse>> {
        let finder = Finder::new(response);
        let filtered_responses_iter = finder.find_iter(response);

        let mut res = Vec::new();

        for m in filtered_responses_iter {
            let begin = m + prefix.len();
            let end = begin + length as usize;

            let content = response[begin as usize..end as usize].to_vec();

            let filtered_response = FilteredResponse {
                begin: begin as u64,
                length,
                bytes: content,
            };

            res.push(filtered_response);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use zktls_program_core::Request;

    use crate::TLSInputBuilder;

    #[tokio::test]
    async fn test_handle_response1() {
        // TODO: We need a new test data
        let bytes = include_str!("../testdata/req0.json");

        let mut req: Request = serde_json::from_str(bytes).unwrap();

        let request_body =
            "GET /get HTTP/1.1\r\nHost: httpbin.org\r\nAccept: */*\r\nConnection: Close\r\n\r\n";

        req.request_info.request = request_body.as_bytes().to_vec().into();

        let mut builder = TLSInputBuilder::new().unwrap();

        let input = builder.handle_request_tls_call(req).await.unwrap();

        println!(
            "response: {}",
            String::from_utf8(input.response.response.clone()).unwrap()
        );

        let guest_input_bytes = serde_json::to_string_pretty(&input).unwrap();

        fs::write("../../target/guest_input0.json", guest_input_bytes).unwrap();
    }
}
