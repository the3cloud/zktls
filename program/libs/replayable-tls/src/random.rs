use once_cell::sync::OnceCell;
use rand_core::{CryptoRng, RngCore};
use rustls_rustcrypto::GeneratedRng;
use std::sync::atomic::{AtomicUsize, Ordering};
static RANDOM: OnceCell<Vec<u8>> = OnceCell::new();

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
pub fn set_random(random: Vec<u8>) {
    RANDOM.set(random).unwrap();
}

static OFFSET: AtomicUsize = AtomicUsize::new(0);

/// A replayable random number generator that uses a predefined sequence of random bytes.
///
/// This struct implements the `RngCore`, `CryptoRng`, and `GeneratedRng` traits,
/// allowing it to be used in contexts where a deterministic random source is needed,
/// such as in testing or replay scenarios.
#[derive(Debug)]
pub struct ReplayableRng;

impl RngCore for ReplayableRng {
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
        let offset = OFFSET.fetch_add(dest.len(), Ordering::SeqCst);

        dest.copy_from_slice(&RANDOM.get().unwrap()[offset..offset + dest.len()]);
    }

    /// Attempts to fill the given byte slice with random bytes.
    ///
    /// This method always succeeds and calls `fill_bytes` internally.
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// Implements the `CryptoRng` marker trait, indicating that this RNG is suitable
/// for cryptographic purposes (in the context of deterministic replay).
impl CryptoRng for ReplayableRng {}

impl GeneratedRng for ReplayableRng {
    /// Creates a new instance of `ReplayableRng`.
    ///
    /// Note that all instances share the same global state defined by `RANDOM` and `OFFSET`.
    fn new() -> Self {
        Self
    }
}
