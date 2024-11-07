use rand_core::{CryptoRng, OsRng, RngCore};
use rustls_rustcrypto::GeneratedRng;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use std::sync::RwLock;

static RANDOM: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Sets the predefined sequence of random bytes to be used by the `ReplayableRng`.
///
/// This function initializes the global `RANDOM` `OnceCell` with the provided vector of bytes.
/// It should be called before any use of `ReplayableRng` to ensure deterministic behavior.
///
/// # Arguments
///
/// * `random` - A `Vec<u8>` containing the sequence of random bytes to be used.
///
/// # Panics
///
/// This function will panic if it's called more than once, as `OnceCell::set` returns an error
/// if the cell has already been initialized.
pub fn set_random_path<P: AsRef<Path>>(random: P) {
    *RANDOM.write().unwrap() = Some(random.as_ref().to_path_buf());
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
    let path = RANDOM.read().unwrap();

    let pp = path.as_ref().unwrap();

    let mut file = OpenOptions::new().create(true).append(true).open(pp)?;

    file.write_all(bytes)?;

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
