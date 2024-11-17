use alloy::sol_types::SolValue;
use t3zktls_core::{GuestInput, GuestOutput};

use crate::request;

pub fn entry(input: &[u8]) -> Vec<u8> {
    let input: GuestInput = ciborium::from_reader(input).expect("Failed to parse input from cbor");

    entry_input(input)
}

pub fn entry_input(input: GuestInput) -> Vec<u8> {
    // println!("input: {:?}", input);
    let GuestOutput {
        response_data,
        request_hash,
    } = request::execute(input.request, input.response);

    (request_hash, response_data).abi_encode()
}

#[cfg(test)]
mod tests {
    use super::entry;

    #[test]
    fn test_entry() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input_bytes = include_bytes!("../testdata/guest_input0.cbor");

        let _output = entry(input_bytes);

        // println!("{:?}", output);
    }
}
