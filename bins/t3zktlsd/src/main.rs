use anyhow::Result;
use clap::Parser;

mod commands;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = commands::Cmd::parse();
    cmd.execute().await
}
