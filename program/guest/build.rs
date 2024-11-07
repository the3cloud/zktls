#[cfg(feature = "r0-backend")]
fn execute() {
    use std::{env, fs, path::Path};

    use risc0_build::embed_methods;

    let guest_list = embed_methods();

    let solidity_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../target/risc0-sol");

    fs::create_dir_all(&solidity_path).unwrap();

    println!("Solidity path: {:?}", solidity_path);

    let option = risc0_build_ethereum::Options::default()
        .with_image_id_sol_path(solidity_path.join("ImageID.sol"))
        .with_elf_sol_path(solidity_path.join("Elf.sol"));

    risc0_build_ethereum::generate_solidity_files(&guest_list, &option).unwrap();
}

#[cfg(feature = "sp1-backend")]
fn execute() {
    use sp1_helper::BuildArgs;

    println!("cargo:rerun-if-changed=build.rs");

    let args = BuildArgs {
        elf_name: "tls.elf".into(),
        ..Default::default()
    };

    sp1_helper::build_program_with_args("tls-sp1", args);
}

#[cfg(not(any(feature = "r0-backend", feature = "sp1-backend")))]
fn execute() {}

fn main() {
    execute();
}
