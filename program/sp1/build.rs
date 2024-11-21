fn execute() {
    use sp1_helper::BuildArgs;

    println!("cargo:rerun-if-changed=build.rs");

    let args = BuildArgs {
        elf_name: "tls.elf".into(),
        ..Default::default()
    };

    sp1_helper::build_program_with_args("tls-sp1", args);
}

fn main() {
    execute();
}
