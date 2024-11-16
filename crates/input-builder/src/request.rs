use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

use anyhow::Result;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use t3zktls_core::{GuestInputRequest, GuestInputResponse};
use t3zktls_recordable_tls::{crypto_provider, time_provider, RecordableStream};

pub fn request_tls_call(request: GuestInputRequest) -> Result<GuestInputResponse> {
    let stream = TcpStream::connect(&request.url)?;
    let mut recordable_stream = RecordableStream::new(stream);

    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let crypto_provider = crypto_provider();
    let time_provider = time_provider();

    let time_provider = Arc::new(time_provider);

    let config =
        ClientConfig::builder_with_details(Arc::new(crypto_provider), time_provider.clone())
            .with_safe_default_protocol_versions()?
            .with_root_certificates(root_store)
            .with_no_client_auth();

    let server_name = String::from(&request.server_name).try_into()?;

    let mut tls_stream = ClientConnection::new(Arc::new(config), server_name)?;

    let mut tls = rustls::Stream::new(&mut tls_stream, &mut recordable_stream);

    let request_data = request.request.data()?;

    println!("request_data: {}", String::from_utf8_lossy(&request_data));

    tls.write_all(&request_data)?;

    let mut buf = Vec::new();
    tls.read_to_end(&mut buf)?;

    recordable_stream.flush()?;

    let random = t3zktls_recordable_tls::random();
    let time = time_provider
        .time()
        .ok_or(anyhow::anyhow!("Time not set"))?;
    let stream_data = recordable_stream.stream_data();

    let mut stream = Vec::new();

    for td in stream_data {
        stream.extend(td.to_bytes());
    }

    Ok(GuestInputResponse {
        time,
        stream,
        random,
        response: buf,
        filtered_responses: vec![],
    })
}

// #[cfg(test)]
// mod tests {

//     use alloy::primitives::Bytes;
//     use t3zktls_core::{GuestInput, Request};

//     use super::*;

//     #[test]
//     fn test_httpbin() -> anyhow::Result<()> {
//         let url = "httpbin.org:443";
//         let data = b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
//         let server_name = "httpbin.org";

//         let request = GuestInputRequest {
//             url: url.to_string(),
//             server_name: server_name.to_string(),
//             request: Request::new_original(data.to_vec()),
//             encrypted_key: Bytes::default(),
//         };

//         let res = request_tls_call(request.clone())?;

//         let _str = String::from_utf8_lossy(&res.response);

//         let input = GuestInput {
//             request,
//             response: res,
//         };

//         let mut output = Vec::new();
//         ciborium::into_writer(&input, &mut output).unwrap();

//         Ok(())
//     }
// }
