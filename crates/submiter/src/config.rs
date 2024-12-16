use alloy::{
    network::TxSigner,
    primitives::{Address, PrimitiveSignature, B256},
    signers::local::LocalSigner,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::RemoteSigner;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub confirmations: u64,
    pub gateway_address: Address,
    pub rpc_url: String,
    pub signer: Signer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Signer {
    Local { private_key: B256 },
    Remote { url: String },
}

impl Signer {
    pub async fn signer(
        &self,
    ) -> Result<Box<dyn TxSigner<PrimitiveSignature> + Send + Sync + 'static>> {
        Ok(match self {
            Signer::Local { private_key } => Box::new(LocalSigner::from_bytes(private_key)?),
            Signer::Remote { url } => Box::new(RemoteSigner::new(url).await),
        })
    }
}
