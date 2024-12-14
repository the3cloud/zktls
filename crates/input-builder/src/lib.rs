mod request;
pub use request::*;

mod handler;
pub use handler::*;

mod regex_cache;

mod config;
pub use config::*;

pub struct FilteredResponse {
    pub begin: u64,
    pub length: u64,
    pub bytes: Vec<u8>,
}
