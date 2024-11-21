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

fn main() {
    execute();
}
