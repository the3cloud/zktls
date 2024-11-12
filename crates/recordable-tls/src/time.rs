use std::{
    sync::RwLock,
    time::{SystemTime, UNIX_EPOCH},
};

use rustls::time_provider::TimeProvider;

// static TIME: RwLock<Option<String>> = RwLock::new(None);

#[derive(Debug, Default)]
pub struct RecordableTimeProvider {
    time: RwLock<Option<String>>,
}

impl RecordableTimeProvider {
    pub fn new() -> Self {
        Self {
            time: RwLock::new(None),
        }
    }

    pub fn time(&self) -> Option<String> {
        self.time.read().unwrap().clone()
    }
}

impl TimeProvider for RecordableTimeProvider {
    fn current_time(&self) -> Option<rustls::pki_types::UnixTime> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let mut time = self.time.write().unwrap();

        *time = Some(format!("{}.{}", now.as_secs(), now.subsec_nanos()));

        Some(rustls::pki_types::UnixTime::since_unix_epoch(now))
    }
}
