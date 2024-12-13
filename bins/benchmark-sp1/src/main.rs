use std::time::Instant;

use t3zktls_core::{
    GuestInput, GuestInputRequest, GuestOutput, GuestProver, Request, TemplateRequest,
};

fn build_input() -> GuestInput {
    let request = GuestInputRequest {
        url: "httpbin.org:443".to_string(),
        // data: b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n".to_vec(),
        server_name: "httpbin.org".to_string(),
        encrypted_key: vec![].into(),
        request: Request::Template(TemplateRequest {
            template: b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n".into(),
            template_hash: Default::default(),
            unencrypted_offset: 0,
            offsets: vec![],
            fields: vec![],
        }),
    };

    let response = t3zktls_input_builder::request_tls_call(request.clone()).unwrap();

    GuestInput { request, response }
}

fn handle_output(guest_output: GuestOutput) {
    let response_data = String::from_utf8(guest_output.response_data).unwrap();
    println!("response_data: {}", response_data);
}

#[tokio::main]
async fn main() {
    let mut prover = t3zktls_guest_prover_sp1::SP1GuestProver::default();

    let input = build_input();

    let start = Instant::now();

    let output = prover.prove(input).await.unwrap();

    let duration = start.elapsed();

    println!("Time taken: {:?}", duration);

    handle_output(output.0);
}
