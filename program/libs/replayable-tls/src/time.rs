use std::time::Duration;

use rustls::time_provider::TimeProvider;

/// A time provider that replays a specific time from a string input.
///
/// This struct implements the `TimeProvider` trait from the `rustls` crate,
/// allowing it to be used in contexts where a deterministic time source is needed,
/// such as in testing or replay scenarios.
#[derive(Debug)]
pub struct ReplayTimeProvider {
    result: rustls::pki_types::UnixTime,
}

impl ReplayTimeProvider {
    /// Creates a new `ReplayTimeProvider` from a time string.
    ///
    /// The time string should be in the format "seconds.nanoseconds".
    ///
    /// # Arguments
    ///
    /// * `time_str` - A string slice that holds the time in the format "seconds.nanoseconds"
    ///
    /// # Returns
    ///
    /// A new instance of `ReplayTimeProvider`
    pub fn new(time_str: &str) -> Self {
        let result = Self::parse_time_file(time_str);
        ReplayTimeProvider { result }
    }

    /// Parses a time string into a `UnixTime` instance.
    ///
    /// # Arguments
    ///
    /// * `time_str` - A string slice that holds the time in the format "seconds.nanoseconds"
    ///
    /// # Returns
    ///
    /// A `UnixTime` instance representing the parsed time
    ///
    /// # Panics
    ///
    /// This function will panic if the input string is not in the correct format
    /// or if the parsing of seconds or nanoseconds fails.
    fn parse_time_file(time_str: &str) -> rustls::pki_types::UnixTime {
        let parts: Vec<&str> = time_str.trim().split('.').collect();

        if parts.len() != 2 {
            panic!("Invalid time format in file");
        }

        let secs = parts[0].parse::<u64>().expect("Failed to parse seconds");
        let nanos = parts[1]
            .parse::<u32>()
            .expect("Failed to parse nanoseconds");

        let duration = Duration::new(secs, nanos);
        rustls::pki_types::UnixTime::since_unix_epoch(duration)
    }
}

impl TimeProvider for ReplayTimeProvider {
    /// Returns the stored time.
    ///
    /// This method always returns `Some` containing the time that was parsed
    /// during the creation of this `ReplayTimeProvider`.
    fn current_time(&self) -> Option<rustls::pki_types::UnixTime> {
        Some(self.result)
    }
}
