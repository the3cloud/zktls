use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cmd {}

impl Cmd {
    pub async fn execute(self) -> Result<()> {
        Ok(())
    }
}
