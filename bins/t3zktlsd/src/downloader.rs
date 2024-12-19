use std::{
    env,
    path::{Path, PathBuf},
};

use alloy::{hex::FromHex, primitives::B256};
use anyhow::Result;
use futures_util::StreamExt;
use t3zktls_core::ZkProver;
use t3zktls_guest_prover_sp1::SP1GuestProver;
use tokio::{fs, io::AsyncWriteExt};

pub async fn download_program(url: &str, path: &Path) -> Result<()> {
    let res = reqwest::get(url).await?;
    let mut file = tokio::fs::File::create(path).await?;

    let mut bytes_stream = res.bytes_stream();

    while let Some(chunk) = bytes_stream.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(())
}

async fn build_guest_path() -> Result<PathBuf> {
    let dir = env::var("HOME")?;
    let dir = Path::new(&dir).join(".local").join("t3zktlsd");

    fs::create_dir_all(&dir).await?;

    Ok(dir)
}

pub async fn build_sp1_prover(mock: bool) -> Result<(impl ZkProver, PathBuf, B256)> {
    let mut guest = SP1GuestProver::default();
    let file = build_guest_path().await?.join("zktls-sp1");

    if mock {
        guest = guest.mock();
    }

    let url = "https://github.com/the3cloud/zkvm-programs/releases/download/v0.1.0-alpha/zktls-sp1";

    if !file.exists() {
        log::info!("downloading program from {}", url);
        download_program(url, &file).await?;
        log::info!("downloaded program success {}", file.display());
    } else {
        log::info!(
            "program already exists {}, if you want to download again, please remove it",
            file.display()
        );
    }

    let pvkey =
        B256::from_hex("0x002c1167a4d8dd15018ac2d333a23e21f6aeaf0e28ff93ad67926588b26fccd4")?;

    Ok((guest, file, pvkey))
}
