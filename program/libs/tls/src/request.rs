use std::{
    io::{Read, Write},
    sync::Arc,
};

use alloy::{
    primitives::{keccak256, Bytes, B256},
    sol_types::SolValue,
};
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use t3zktls_core::{GuestInputRequest, GuestInputResponse, GuestOutput, Request};
use t3zktls_replayable_tls::{crypto_provider, set_random, ReplayStream, ReplayTimeProvider};

pub fn execute(request: GuestInputRequest, response: GuestInputResponse) -> GuestOutput {
    let mut stream = ReplayStream::new(response.stream.clone());
    let time_provider = ReplayTimeProvider::new(&response.time);
    set_random(response.random);

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let crypto_provider = crypto_provider();

    let config =
        ClientConfig::builder_with_details(Arc::new(crypto_provider), Arc::new(time_provider))
            .with_safe_default_protocol_versions()
            .expect("Failed to set protocol versions")
            .with_root_certificates(root_store)
            .with_no_client_auth();

    let server_name = String::from(&request.server_name)
        .try_into()
        .expect("Failed to convert server name");

    let mut tls_stream =
        ClientConnection::new(Arc::new(config), server_name).expect("Failed to create TLS stream");

    let mut tls = rustls::Stream::new(&mut tls_stream, &mut stream);

    let request_data = request.request.data().expect("Failed to get request data");

    tls.write_all(&request_data).expect("Failed to write data");

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf).expect("Failed to read data");

    let mut serialized_request = Vec::new();
    ciborium::into_writer(&request, &mut serialized_request).expect("Failed to serialize request");

    let request_hash = compute_request_hash(
        request.url,
        request.server_name,
        request.encrypted_key,
        request.request,
    );

    GuestOutput {
        response_data: buf,
        request_hash,
    }
}

fn compute_request_hash(
    remote: String,
    server_name: String,
    encrypted_key: Bytes,
    request: Request,
) -> [u8; 32] {
    match request {
        Request::Template(template_request) => compute_template_request_hash(
            remote,
            server_name,
            encrypted_key,
            template_request.template_hash,
            &template_request.offsets[..template_request.unencrypted_offset as usize],
            &template_request.fields[..template_request.unencrypted_offset as usize],
        ),
    }
}

fn compute_template_request_hash(
    remote: String,
    server_name: String,
    encrypted_key: Bytes,
    template_hash: B256,
    offsets: &[u64],
    fields: &[Bytes],
) -> [u8; 32] {
    let data = (
        remote,
        server_name,
        encrypted_key,
        template_hash,
        offsets,
        fields,
    );

    keccak256(data.abi_encode()).into()
}
