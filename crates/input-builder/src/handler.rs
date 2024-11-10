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
    pub fn new(config: Config) -> Self {
        Self {
            cache: RegexCache::new(config.regex_cache_size),
        }
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
            data: req.request.data()?.to_vec(),
        };

        let guest_input_request_cloned = guest_input_request.clone();

        let mut guest_input_response =
            tokio::task::spawn_blocking(move || request_tls_call(guest_input_request_cloned))
                .await??;

        let response = guest_input_response.response.clone();

        let mut filtered_responses = Vec::new();

        match req.response_template {
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
        // use xrust::transform::context::StaticContextBuilder;
        // use xrust::Error;
        // use xrust::ErrorKind;

        // let xpath = xrust::parser::xpath::parse::<RNode>(&xpath, None)
        //     .map_err(|_| anyhow::anyhow!("invalid xpath"))?;

        // let root = RNode::new_document();
        // xrust::parser::xml::parse(root.clone(), &response, None)
        //     .map_err(|_| anyhow::anyhow!("invalid xml"))?;

        // let context = ContextBuilder::new()
        //     .context(vec![Item::Node(root)])
        //     .build();

        // let mut stctxt = StaticContextBuilder::new()
        //     .message(|_| Ok(()))
        //     .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        //     .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
        //     .build();

        // let _result = context
        //     .dispatch(&mut stctxt, &xpath)
        //     .map_err(|_| anyhow::anyhow!("invalid xpath match"))?;

        // Ok(Vec::new())

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
