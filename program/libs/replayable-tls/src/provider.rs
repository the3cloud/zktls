use rustls::{
    crypto::{CryptoProvider, SupportedKxGroup},
    time_provider::TimeProvider,
};
use rustls_rustcrypto::{all_cipher_suites, all_signature_verification_algorithms, kx, Provider};

use crate::{random::ReplayableRng, ReplayTimeProvider};

/// Provides a customized `CryptoProvider` for use with rustls.
///
/// This function creates and returns a `CryptoProvider` instance configured with:
/// - All available cipher suites
/// - Key exchange groups: X25519, SecP256R1, and SecP384R1
/// - All available signature verification algorithms
/// - A custom key provider and secure random source using `ReplayableRng`
///
/// # Returns
///
/// A `CryptoProvider` instance configured for use in a deterministic or replay scenario.
pub fn crypto_provider() -> CryptoProvider {
    static RUSTCRYPTO_PROVIDER: Provider<ReplayableRng> = Provider::<ReplayableRng>::new();
    const ALL_KX_GROUPS: &[&dyn SupportedKxGroup] = &[
        &kx::X25519::<ReplayableRng>::new(),
        &kx::SecP256R1::<ReplayableRng>::new(),
        &kx::SecP384R1::<ReplayableRng>::new(),
    ];

    CryptoProvider {
        cipher_suites: all_cipher_suites(),
        kx_groups: ALL_KX_GROUPS.to_vec(),
        signature_verification_algorithms: all_signature_verification_algorithms(),
        key_provider: &RUSTCRYPTO_PROVIDER,
        secure_random: &RUSTCRYPTO_PROVIDER,
    }
}

/// Creates a `TimeProvider` that replays a specific time.
///
/// This function takes a time string and returns a `ReplayTimeProvider` instance,
/// which implements the `TimeProvider` trait. This is useful for scenarios where
/// deterministic time behavior is required, such as in testing or replay situations.
///
/// # Arguments
///
/// * `time_str` - A string slice representing the time to be replayed, in the format "seconds.nanoseconds"
///
/// # Returns
///
/// An implementation of `TimeProvider` that always returns the specified time.
pub fn time_provider(time_str: &str) -> impl TimeProvider {
    ReplayTimeProvider::new(time_str)
}
