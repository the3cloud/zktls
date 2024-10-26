use alloy::primitives::Bytes;
use anyhow::Result;
use k256::{elliptic_curve::ecdh, PublicKey, Secp256k1, SecretKey};
use t3_zktls_listeners_ethereum::TLSDataDecoder;

pub struct K256LocalDecryptor {
    private_key: SecretKey,
}

impl TLSDataDecoder for K256LocalDecryptor {
    async fn decode_tls_data(&self, data: &mut Bytes, encrypted_key: &Bytes) -> Result<()> {
        let public_key = PublicKey::from_sec1_bytes(encrypted_key)?;

        let shared_secret =
            ecdh::diffie_hellman(self.private_key.to_nonzero_scalar(), public_key.as_affine());

        let key = shared_secret.raw_secret_bytes();

        Ok(())
    }
}
