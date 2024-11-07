use aes_gcm::{aead::AeadMutInPlace, Aes256Gcm, KeyInit};
use anyhow::{Ok, Result};
use k256::{
    elliptic_curve::{ecdh, rand_core::OsRng},
    sha2::{Digest, Sha256},
    PublicKey, SecretKey,
};

pub struct K256LocalEncryptor {
    shared_secret: [u8; 32],
    state: [u8; 32],
    pub public_key: Vec<u8>,
}

impl K256LocalEncryptor {
    pub fn new(encrypted_public_key: &[u8]) -> Result<Self> {
        let private_key = SecretKey::random(&mut OsRng);

        let public_key = PublicKey::from_sec1_bytes(encrypted_public_key)?;

        let shared_secret =
            ecdh::diffie_hellman(private_key.to_nonzero_scalar(), public_key.as_affine());

        let key = shared_secret.raw_secret_bytes();
        log::trace!("encryptor key: {:?}", key);

        let state = Sha256::digest(key);

        Ok(Self {
            shared_secret: (*key).into(),
            state: state.into(),
            public_key: private_key.public_key().to_sec1_bytes().to_vec(),
        })
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) -> Result<()> {
        let mut cipher = Aes256Gcm::new(&self.shared_secret.into());

        let iv = (&self.state[..12]).into();

        log::trace!("encrypt iv: {:?}", iv);

        cipher
            .encrypt_in_place(iv, &[], data)
            .map_err(|e| anyhow::anyhow!("failed to encrypt: {}", e))?;

        let mut hasher = Sha256::default();
        hasher.update(self.state);
        hasher.update(data);
        let hash = hasher.finalize();

        self.state = hash.into();

        Ok(())
    }
}

// pub fn encrypt_data(encrypt_public_key: &[u8], data: &mut Vec<u8>) -> Result<()> {
//     let private_key = SecretKey::random(&mut OsRng);

//     let public_key = PublicKey::from_sec1_bytes(encrypt_public_key)?;

//     let shared_secret =
//         ecdh::diffie_hellman(private_key.to_nonzero_scalar(), public_key.as_affine());

//     let key = shared_secret.raw_secret_bytes();

//     let mut cipher = Aes256Gcm::new(key);

//     let iv = Sha256::digest(key);

//     cipher
//         .encrypt_in_place((&iv[..12]).into(), &[], data)
//         .map_err(|e| anyhow::anyhow!("failed to encrypt: {:?}", e))?;

//     Ok(())
// }
