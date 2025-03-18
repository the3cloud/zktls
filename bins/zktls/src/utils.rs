use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use futures_util::StreamExt;
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

pub async fn get_sp1_program() -> Result<Vec<u8>> {
    let file = build_guest_path().await?.join("zktls-sp1");

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
    let program = fs::read(&file).await?;

    Ok(program)
}

pub async fn get_r0_program() -> Result<Vec<u8>> {
    let file = build_guest_path().await?.join("zktls-r0");

    let url = "https://github.com/the3cloud/zkvm-programs/releases/download/v0.1.0-alpha/zktls-r0";

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
    let program = fs::read(&file).await?;

    Ok(program)
}
