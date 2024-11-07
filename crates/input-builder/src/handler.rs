use std::num::NonZeroUsize;

use anyhow::Result;
use t3zktls_core::{
    GuestInput, GuestInputRequest, ProveRequest, RequestTLSCallHandler, ResponseTemplate,
};

use crate::{regex_cache::RegexCache, request::request_tls_call};

pub struct InputBuilder {
    cache: RegexCache,
}

impl InputBuilder {
    pub fn new(size: NonZeroUsize) -> Self {
        Self {
            cache: RegexCache::new(size),
        }
    }
}

impl RequestTLSCallHandler for InputBuilder {
    async fn handle_request_tls_call(&mut self, req: ProveRequest) -> Result<()> {
        let guest_input_request = GuestInputRequest {
            url: req.remote.url,
            data: req.remote.data,
            server_name: req.remote.server_name,
        };

        let mut guest_input_response = request_tls_call(&guest_input_request)?;

        // Based on response, splited data use
        let response_template = ResponseTemplate::new(req.response_template)?;
        let regex = response_template.regex();

        let response_str = String::from_utf8(guest_input_response.response.clone())?;

        let filtered_responses = self
            .cache
            .find(req.response_template_id, regex, &response_str)?;

        guest_input_response.filtered_responses = filtered_responses;

        let _guest_input = GuestInput {
            request: guest_input_request,
            response: guest_input_response,
        };

        Ok(())
    }
}
