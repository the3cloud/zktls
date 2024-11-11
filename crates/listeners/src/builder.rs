use std::collections::HashMap;

use alloy::primitives::{
    bytes::{BufMut, BytesMut},
    Bytes, B256,
};
use anyhow::Result;
use t3zktls_contracts_ethereum::IZkTLSGateway::{
    RequestTLSCallBegin, RequestTLSCallSegment, RequestTLSCallTemplateField,
};
use t3zktls_core::{
    OriginalRequest, ProveRequest, Request, ResponseTemplate, TLSDataDecryptor,
    TLSDataDecryptorGenerator, TemplateRequest,
};

pub struct RequestBuilder<'a, D> {
    prover_id: B256,
    request_id: Option<B256>,
    remote: Option<String>,
    server_name: Option<String>,
    encrypted_key: Option<Bytes>,
    max_response_size: u64,
    request_template_id: Option<B256>,
    response_template_id: Option<B256>,

    original_request: Option<BytesMut>,
    template_request: Option<TemplateRequest>,
    response_template: ResponseTemplate,

    decryptor: &'a D,
}

impl<'a, D> RequestBuilder<'a, D> {
    pub fn new(prover_id: B256, decryptor: &'a D) -> Self {
        Self {
            prover_id,
            request_id: None,
            remote: None,
            server_name: None,
            decryptor,
            encrypted_key: None,
            max_response_size: 0,
            request_template_id: None,
            response_template_id: None,
            original_request: None,
            template_request: None,
            response_template: ResponseTemplate::None,
        }
    }
}

impl<'a, D> RequestBuilder<'a, D>
where
    D: TLSDataDecryptorGenerator,
{
    pub fn add_request_from_begin_logs(&mut self, log: RequestTLSCallBegin) -> Result<()> {
        let prover_id = self.prover_id;

        if prover_id != self.prover_id {
            return Err(anyhow::anyhow!("prover id mismatch"));
        }

        self.request_id = Some(log.requestId);

        self.remote = Some(log.remote);
        self.server_name = Some(log.serverName);
        self.encrypted_key = Some(log.encryptedKey);
        self.max_response_size = log.maxResponseSize;
        self.request_template_id = Some(log.requestTemplateHash);
        self.response_template_id = Some(log.responseTemplateHash);

        log::trace!(
            "\nrequest_id: {:?}\n remote: {:?}\n server_name: {:?}\n encrypted_key: {:?}\n max_response_size: {:?}\n request_template_id: {:?}\n response_template_id: {:?}",
            self.request_id,
            self.remote,
            self.server_name,
            self.encrypted_key,
            self.max_response_size,
            self.request_template_id,
            self.response_template_id,
        );

        Ok(())
    }

    pub fn add_request_template(&mut self, template: &Bytes) -> Result<()> {
        let template_request = TemplateRequest {
            template: t3zktls_core::template::parse_request_template(template.clone())?,
            fields: HashMap::new(),
        };

        self.template_request = Some(template_request);

        Ok(())
    }

    pub fn add_response_template(&mut self, template: &Bytes) -> Result<()> {
        let response_template = t3zktls_core::template::parse_response_template(template.clone())?;

        self.response_template = response_template;

        Ok(())
    }

    pub async fn add_request_from_segment_logs(
        &mut self,
        log: RequestTLSCallSegment,
    ) -> Result<()> {
        let append_data = if log.isEncrypted {
            let mut decryptor = self
                .decryptor
                .generate_decryptor(
                    self.encrypted_key
                        .as_ref()
                        .ok_or(anyhow::anyhow!("encrypted key is not set"))?,
                )
                .await?;

            let mut data = log.data.to_vec();

            decryptor.decrypt_tls_data(&mut data).await?;
            data.into()
        } else {
            log.data
        };

        log::trace!("append_data: {:?}", append_data);

        if let Some(original_request) = &mut self.original_request {
            original_request.put(append_data);
        } else {
            let mut data = BytesMut::new();
            data.put(append_data);
            self.original_request = Some(data);
        }

        Ok(())
    }

    pub async fn add_request_from_template_field_logs(
        &mut self,
        log: RequestTLSCallTemplateField,
    ) -> Result<()> {
        let append_data = if log.isEncrypted {
            let mut decryptor = self
                .decryptor
                .generate_decryptor(
                    self.encrypted_key
                        .as_ref()
                        .ok_or(anyhow::anyhow!("encrypted key is not set"))?,
                )
                .await?;

            let mut data = log.value.to_vec();

            decryptor.decrypt_tls_data(&mut data).await?;
            data.into()
        } else {
            log.value
        };

        if let Some(template_request) = &mut self.template_request {
            template_request.fields.insert(log.field, append_data);
        } else {
            return Err(anyhow::anyhow!("request is not a template"));
        }

        Ok(())
    }

    pub fn build(self) -> Result<ProveRequest> {
        let request = if let Some(template_request) = self.template_request {
            Request::Template(template_request)
        } else {
            let data = self
                .original_request
                .ok_or(anyhow::anyhow!("original request is not set"))?
                .freeze()
                .into();

            Request::Original(OriginalRequest { data })
        };

        Ok(ProveRequest {
            request_id: self
                .request_id
                .ok_or(anyhow::anyhow!("request id is not set"))?,
            prover_id: self.prover_id,
            remote: self.remote.ok_or(anyhow::anyhow!("remote is not set"))?,
            server_name: self
                .server_name
                .ok_or(anyhow::anyhow!("server name is not set"))?,
            max_response_size: self.max_response_size,
            response_template_id: self
                .response_template_id
                .ok_or(anyhow::anyhow!("response template id is not set"))?,
            response_template: self.response_template,
            request,
        })
    }
}
