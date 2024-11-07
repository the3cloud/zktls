use std::path::Path;

use rustls::{
    crypto::{CryptoProvider, SupportedKxGroup},
    time_provider::TimeProvider,
};
use rustls_rustcrypto::{all_cipher_suites, all_signature_verification_algorithms, kx, Provider};

use crate::{random::RecordableRng, time::RecordableTimeProvider};

pub fn crypto_provider() -> CryptoProvider {
    static RUSTCRYPTO_PROVIDER: Provider<RecordableRng> = Provider::<RecordableRng>::new();
    const ALL_KX_GROUPS: &[&dyn SupportedKxGroup] = &[
        &kx::X25519::<RecordableRng>::new(),
        &kx::SecP256R1::<RecordableRng>::new(),
        &kx::SecP384R1::<RecordableRng>::new(),
    ];

    CryptoProvider {
        cipher_suites: all_cipher_suites(),
        kx_groups: ALL_KX_GROUPS.to_vec(),
        signature_verification_algorithms: all_signature_verification_algorithms(),
        secure_random: &RUSTCRYPTO_PROVIDER,
        key_provider: &RUSTCRYPTO_PROVIDER,
    }
}

pub fn time_provider<P: AsRef<Path>>(path: P) -> impl TimeProvider {
    RecordableTimeProvider::new(path.as_ref().to_path_buf())
}
