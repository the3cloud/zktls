use std::{
    io::{Read, Write},
    sync::Arc,
};

use rustls::{ClientConfig, ClientConnection, RootCertStore};
use sha2::{Digest, Sha256};
use t3zktls_core::{GuestInputRequest, GuestInputResponse, GuestOutput};
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

    let request_hash = Sha256::digest(&serialized_request);

    GuestOutput {
        response_data: buf,
        request_hash: request_hash.into(),
    }
}
