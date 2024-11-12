use rand_core::{CryptoRng, OsRng, RngCore};
use rustls_rustcrypto::GeneratedRng;

use std::sync::RwLock;

static RANDOM: RwLock<Vec<u8>> = RwLock::new(Vec::new());

pub fn random() -> Vec<u8> {
    RANDOM.read().unwrap().clone()
}

/// A replayable random number generator that uses a predefined sequence of random bytes.
///
/// This struct implements the `RngCore`, `CryptoRng`, and `GeneratedRng` traits,
/// allowing it to be used in contexts where a deterministic random source is needed,
/// such as in testing or replay scenarios.
#[derive(Debug)]
pub struct RecordableRng;

impl RngCore for RecordableRng {
    /// Returns a fixed value of 0 for u32.
    ///
    /// This method is not implemented to use the predefined random sequence.
    fn next_u32(&mut self) -> u32 {
        0
    }

    /// Returns a fixed value of 0 for u64.
    ///
    /// This method is not implemented to use the predefined random sequence.
    fn next_u64(&mut self) -> u64 {
        0
    }

    /// Fills the given byte slice with random bytes from the predefined sequence.
    ///
    /// This method uses an atomic offset to keep track of the current position
    /// in the random sequence, ensuring thread-safe access.
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        OsRng.fill_bytes(dest);

        append_bytes_to_file(dest).unwrap();
    }

    /// Attempts to fill the given byte slice with random bytes.
    ///
    /// This method always succeeds and calls `fill_bytes` internally.
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn append_bytes_to_file(bytes: &[u8]) -> std::io::Result<()> {
    let mut random = RANDOM.write().unwrap();

    random.extend_from_slice(bytes);

    Ok(())
}

/// Implements the `CryptoRng` marker trait, indicating that this RNG is suitable
/// for cryptographic purposes (in the context of deterministic replay).
impl CryptoRng for RecordableRng {}

impl GeneratedRng for RecordableRng {
    /// Creates a new instance of `ReplayableRng`.
    ///
    /// Note that all instances share the same global state defined by `RANDOM` and `OFFSET`.
    fn new() -> Self {
        Self
    }
}
