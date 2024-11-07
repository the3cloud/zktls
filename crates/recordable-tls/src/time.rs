use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use rustls::time_provider::TimeProvider;

#[derive(Debug)]
pub struct RecordableTimeProvider {
    path: PathBuf,
}

impl RecordableTimeProvider {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl TimeProvider for RecordableTimeProvider {
    fn current_time(&self) -> Option<rustls::pki_types::UnixTime> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        fs::write(
            &self.path,
            format!("{}.{}", now.as_secs(), now.subsec_nanos()),
        )
        .unwrap();

        Some(rustls::pki_types::UnixTime::since_unix_epoch(now))
    }
}
