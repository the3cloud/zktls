use anyhow::Result;
use clap::Parser;

mod commands;
mod config;

mod downloader;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cmd = commands::Cmd::parse();
    cmd.execute().await
}
