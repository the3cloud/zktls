use alloy::{
    consensus::SignableTransaction,
    network::TxSigner,
    primitives::{Address, PrimitiveSignature},
    signers::Result,
};

pub struct RemoteSigner {
    url: String,
}

impl RemoteSigner {
    pub async fn new(url: &str) -> Self {
        Self { url: url.into() }
    }
}

#[async_trait::async_trait]
impl TxSigner<PrimitiveSignature> for RemoteSigner {
    fn address(&self) -> Address {
        let _url = &self.url;

        todo!()
    }

    async fn sign_transaction(
        &self,
        _tx: &mut dyn SignableTransaction<PrimitiveSignature>,
    ) -> Result<PrimitiveSignature> {
        todo!()
    }
}
