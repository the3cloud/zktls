use std::{env, path::Path, process::Command};

fn main() {
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    let contracts_dir = Path::new(&project_root).to_path_buf();

    println!("cargo:rerun-if-changed={}", contracts_dir.display());

    let output = Command::new("forge")
        .current_dir(&contracts_dir)
        .arg("build")
        .arg("-o")
        .arg(
            contracts_dir
                .join("../target/contracts")
                .display()
                .to_string(),
        )
        .output()
        .expect("Failed to execute forge build command");

    if !output.status.success() {
        panic!(
            "Forge build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
