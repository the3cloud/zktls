use aes_gcm::{aead::AeadMutInPlace, Aes256Gcm, KeyInit};
use alloy::primitives::Bytes;
use anyhow::Result;
use k256::{
    elliptic_curve::{ecdh, rand_core::OsRng},
    sha2::{Digest, Sha256},
    PublicKey, SecretKey,
};
use t3zktls_core::{TLSDataDecryptor, TLSDataDecryptorGenerator};
// use t3zktls_listeners_ethereum::{TLSDataDecryptor, TLSDataDecryptorGenerator};

/// K256LocalDecryptor is a struct that implements the TLSDataDecryptor trait.
/// It uses the K-256 elliptic curve for key exchange and AES-GCM for symmetric encryption.
///
/// This decryptor is designed to work with locally stored private keys for decrypting TLS data.
/// It performs Elliptic Curve Diffie-Hellman (ECDH) key exchange to derive a shared secret,
/// which is then used to decrypt the data using AES-GCM.
#[derive(Clone)]
pub struct K256LocalDecryptorGenerator {
    private_key: SecretKey,
}

impl TLSDataDecryptorGenerator for K256LocalDecryptorGenerator {
    type Decryptor = K256LocalDecryptor;

    async fn generate_decryptor(&self, encrypted_public_key: &Bytes) -> Result<Self::Decryptor> {
        let public_key = PublicKey::from_sec1_bytes(&encrypted_public_key[0..33])?;

        let shared_secret =
            ecdh::diffie_hellman(self.private_key.to_nonzero_scalar(), public_key.as_affine());

        log::trace!("decryptor key: {:?}", shared_secret.raw_secret_bytes());

        let state = Sha256::digest(shared_secret.raw_secret_bytes());

        Ok(K256LocalDecryptor {
            shared_secret: (*shared_secret.raw_secret_bytes()).into(),
            state: state.into(),
        })
    }
}

impl K256LocalDecryptorGenerator {
    /// Generates a new K256LocalDecryptor with a randomly generated private key.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new K256LocalDecryptor instance or an error if key generation fails.
    pub fn generate_key() -> Result<Self> {
        let private_key = SecretKey::random(&mut OsRng);
        Ok(Self { private_key })
    }

    /// Creates a new K256LocalDecryptor from an existing private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - A byte slice containing the private key.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new K256LocalDecryptor instance or an error if key creation fails.
    pub fn new(private_key: &[u8]) -> Result<Self> {
        Ok(Self {
            private_key: SecretKey::from_bytes(private_key.into())?,
        })
    }

    /// Exports the private key associated with this K256LocalDecryptor.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>>` - The private key as a vector of bytes, or an error if key extraction fails.
    pub fn export_private_key(&self) -> Result<Vec<u8>> {
        Ok(self.private_key.to_bytes().to_vec())
    }

    /// Retrieves the public key associated with this K256LocalDecryptor.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>>` - The public key as a vector of bytes in SEC1 format, or an error if key extraction fails.
    pub fn encrypt_public_key(&self) -> Result<Vec<u8>> {
        let public_key = self.private_key.public_key();
        Ok(public_key.to_sec1_bytes().to_vec())
    }
}

pub struct K256LocalDecryptor {
    shared_secret: [u8; 32],
    state: [u8; 32],
}

impl TLSDataDecryptor for K256LocalDecryptor {
    async fn decrypt_tls_data(&mut self, data: &mut Vec<u8>) -> Result<()> {
        let mut key = Aes256Gcm::new(&self.shared_secret.into());

        let iv = (&self.state[..12]).into();

        log::trace!("decrypt iv: {:?}", iv);

        let mut hasher = Sha256::default();
        hasher.update(self.state);
        hasher.update(&data);

        key.decrypt_in_place(iv, &[], data)
            .map_err(|e| anyhow::anyhow!("failed to decrypt: {:?}", e))?;

        let hash = hasher.finalize();

        self.state = hash.into();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{K256LocalDecryptorGenerator, K256LocalEncryptor};
    use alloy::{
        network::Ethereum,
        node_bindings::Anvil,
        primitives::{Bytes, B256},
        providers::{ReqwestProvider, RootProvider},
        transports::http::reqwest::Url,
    };
    use t3zktls_contracts_ethereum::ZkTLSGateway;

    use t3zktls_listeners_ethereum::{Config, Listener};

    #[tokio::test]
    async fn test_decryptor() {
        let _ = env_logger::builder().is_test(true).try_init();

        let anvil = Anvil::new().spawn();

        let url = format!("http://localhost:{}", anvil.port());

        let decryptor = K256LocalDecryptorGenerator::generate_key().unwrap();
        let encrypt_public_key = decryptor.encrypt_public_key().unwrap();

        let mut encryptor = K256LocalEncryptor::new(&encrypt_public_key).unwrap();

        let data0 = b"GET /get HTTP/1.1\r\nHost: ";

        let mut data1 = b"httpbin.org".to_vec();
        encryptor.encrypt(&mut data1).unwrap();

        let data2 = b"\r\nUser-Agent: ";

        let mut data3 = b"zkTLS0.1".to_vec();
        encryptor.encrypt(&mut data3).unwrap();

        let data4 = b"\r\nAccept: */*\r\n";

        let provider: RootProvider<_, Ethereum> =
            ReqwestProvider::new_http(Url::parse(&url).unwrap());

        let zk_tls_gateway = ZkTLSGateway::deploy(provider.clone()).await.unwrap();

        let gateway_address = *zk_tls_gateway.address();

        zk_tls_gateway
            .requestTLSCall(
                "httpbin.org:443".into(),
                "httpbin.org".into(),
                Bytes::from(encryptor.public_key),
                vec![
                    Bytes::from(data0),
                    Bytes::from(data1),
                    Bytes::from(data2),
                    Bytes::from(data3),
                    Bytes::from(data4),
                ],
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        let config = Config {
            gateway_address,
            begin_block_number: 0,
            block_number_batch_size: 100,
            prover_id: B256::from_str(
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            sleep_duration: 1,
        };

        let mut listener = Listener::new(Some(1), config, provider, (), decryptor);

        listener.pull_blocks().await.unwrap();
    }
}
