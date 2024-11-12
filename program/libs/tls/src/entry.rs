use t3zktls_core::{GuestInput, GuestOutput};

use crate::request;

pub fn entry(input: &[u8]) -> Vec<u8> {
    let input: GuestInput = ciborium::from_reader(input).expect("Failed to parse input from cbor");

    let res = request::execute(input.request, input.response);

    let mut result_bytes = Vec::new();

    ciborium::into_writer(&res, &mut result_bytes).expect("Failed to serialize output to cbor");

    result_bytes
}

pub fn entry_input(input: GuestInput) -> GuestOutput {
    request::execute(input.request, input.response)
}

#[cfg(test)]
mod tests {
    use super::entry;

    #[test]
    fn test_entry() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input_bytes = include_bytes!("../testdata/guest_input0.cbor");

        let output = entry(input_bytes);

        println!("{:?}", output);
    }
}
