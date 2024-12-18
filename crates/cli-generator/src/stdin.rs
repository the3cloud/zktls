use anyhow::Result;
use t3zktls_core::RequestGenerator;
use t3zktls_program_core::Request;
use tokio::io::{self, AsyncBufReadExt};

#[derive(Debug, Default)]
pub struct StdinGenerator {}

impl StdinGenerator {
    pub async fn requests() -> Result<Vec<Request>> {
        let stdin = io::stdin();
        let reader = io::BufReader::new(stdin);
        let mut lines = reader.lines();

        let mut requests = Vec::new();
        while let Some(line) = lines.next_line().await? {
            let request: Request = serde_json::from_str(&line)?;
            requests.push(request);
        }

        Ok(requests)
    }
}

impl RequestGenerator for StdinGenerator {
    async fn generate_requests(&mut self) -> Result<Vec<Request>> {
        StdinGenerator::requests().await
    }
}
