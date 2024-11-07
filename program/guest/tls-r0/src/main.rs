use t3zktls_core::GuestInput;

fn main() {
    let input: GuestInput = risc0_zkvm::guest::env::read();

    let output = t3zktls_program_tls::entry_input(input);

    risc0_zkvm::guest::env::commit(&output);
}
