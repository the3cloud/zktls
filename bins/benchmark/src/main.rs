use std::time::Instant;

use t3zktls_core::{GuestInput, GuestInputRequest, GuestOutput, Request, TemplateRequest};

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

pub fn build_input_bytes() -> Vec<u8> {
    let guest_input = build_input();

    let mut input_bytes = Vec::new();
    ciborium::into_writer(&guest_input, &mut input_bytes).unwrap();

    input_bytes
}

fn handle_output(guest_output: GuestOutput) {
    let response_data = String::from_utf8(guest_output.response_data).unwrap();
    println!("response_data: {}", response_data);
}

pub fn handle_output_bytes(output_bytes: &[u8]) {
    let guest_output: GuestOutput = ciborium::from_reader(output_bytes).unwrap();

    handle_output(guest_output)
}

#[cfg(feature = "sp1-backend")]
fn main() {
    use sp1_sdk::{ProverClient, SP1Stdin};

    sp1_sdk::utils::setup_logger();

    let client = ProverClient::new();

    let mut stdin = SP1Stdin::new();

    let input_bytes = build_input_bytes();

    stdin.write_vec(input_bytes);

    let start = Instant::now();

    let (pk, vk) = client.setup(t3zktls_program::TLS_ELF);

    let proof = client.prove(&pk, stdin).groth16().run().unwrap();

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    let output_reader = proof.public_values.as_slice();

    handle_output_bytes(output_reader);

    client.verify(&proof, &vk).unwrap();
}

#[cfg(feature = "r0-backend")]
fn main() {
    use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
    use t3zktls_program::TLS_R0_ELF;

    env_logger::init();

    let prover = default_prover();

    let input = build_input();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let start = Instant::now();
    let prove_result = prover
        .prove_with_opts(env, TLS_R0_ELF, &ProverOpts::groth16())
        .unwrap();

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    let journal = prove_result.receipt.journal;
    let guest_output: GuestOutput = journal.decode().unwrap();

    handle_output(guest_output);
}
