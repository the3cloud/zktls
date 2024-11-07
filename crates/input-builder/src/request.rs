use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use anyhow::Result;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use t3zktls_core::{GuestInputRequest, GuestInputResponse};
use t3zktls_recordable_tls::{crypto_provider, time_provider, RecordableStream};

pub fn request_tls_call(request: &GuestInputRequest) -> Result<GuestInputResponse> {
    let temp_dir = tempfile::tempdir()?;
    let time_path = temp_dir.path().join("time");
    let stream_path = temp_dir.path().join("stream");
    let random_path = temp_dir.path().join("random");

    t3zktls_recordable_tls::set_random_path(&random_path);

    let stream = TcpStream::connect(&request.url)?;
    let mut recordable_stream = RecordableStream::new(stream, File::create(&stream_path)?);

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let crypto_provider = crypto_provider();
    let time_provider = time_provider(time_path.clone());

    let config =
        ClientConfig::builder_with_details(Arc::new(crypto_provider), Arc::new(time_provider))
            .with_safe_default_protocol_versions()?
            .with_root_certificates(root_store)
            .with_no_client_auth();

    let server_name = String::from(&request.server_name).try_into()?;

    let mut tls_stream = ClientConnection::new(Arc::new(config), server_name)?;

    let mut tls = rustls::Stream::new(&mut tls_stream, &mut recordable_stream);

    tls.write_all(&request.data)?;

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf)?;

    // read data to state
    let random = fs::read(random_path)?;
    let time = fs::read_to_string(time_path)?;
    let stream = fs::read(stream_path)?;

    Ok(GuestInputResponse {
        time,
        stream,
        random,
        response: buf,
        filtered_responses: vec![],
    })
}

#[cfg(test)]
mod tests {

    use t3zktls_core::GuestInput;

    use super::*;

    #[test]
    fn test_httpbin() -> anyhow::Result<()> {
        let url = "httpbin.org:443";
        let data = b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
        let server_name = "httpbin.org";

        let request = GuestInputRequest {
            url: url.to_string(),
            server_name: server_name.to_string(),
            data: data.to_vec(),
        };

        // let mut state = TLSRequestState::default();
        let res = request_tls_call(&request)?;

        let _str = String::from_utf8_lossy(&res.response);

        let input = GuestInput {
            request,
            response: res,
        };

        let mut output = Vec::new();
        ciborium::into_writer(&input, &mut output).unwrap();
        let target = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/input.cbor");
        fs::write(target, output)?;

        Ok(())
    }
}
