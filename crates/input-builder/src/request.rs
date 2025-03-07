use std::{
    io::{Read, Write},
    net::TcpStream,
    panic,
    sync::Arc,
};

use anyhow::Result;
use rustls::{ClientConfig, ClientConnection, RootCertStore};
use t3zktls_program_core::{GuestInputResponse, Request, ResponseTemplate};
use t3zktls_recordable_tls::{crypto_provider, time_provider, RecordableStream};

pub fn request_tls_call(request: &Request) -> Result<GuestInputResponse> {
    let res = panic::catch_unwind(move || _request_tls_call(request))
        .map_err(|e| anyhow::anyhow!("{:?}", e))??;

    Ok(res)
}

fn _request_tls_call(request: &Request) -> Result<GuestInputResponse> {
    let stream = TcpStream::connect(&request.remote_addr)?;
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

    let request_data = request.request.as_ref();

    tls.write_all(&request_data)?;

    let mut response = Vec::new();
    tls.read_to_end(&mut response)?;
    tls.flush()?;

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

    let mut filtered_responses_begin = Vec::new();
    let mut filtered_responses_length = Vec::new();
    let mut filtered_responses = Vec::new();

    for template in &request.response_template {
        match template {
            ResponseTemplate::Offset { begin, length } => {
                let begin = *begin;
                let length = *length;

                filtered_responses_begin.push(begin);
                filtered_responses_length.push(length);

                let begin = begin as usize;
                let length = length as usize;

                let res = response.get(begin..begin + length).ok_or(anyhow::anyhow!("Offset out of range"))?.to_vec();
                filtered_responses.push(res.into());
            }
            ResponseTemplate::Regex { pattern: _ } => {
                // filtered_responses.push(pattern.clone());
            }
        }
    }

    Ok(GuestInputResponse {
        time,
        stream,
        random,
        response,
        filtered_responses_begin,
        filtered_responses_length,
        filtered_responses,
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
