#[cfg(feature = "r0-backend")]
mod __r0 {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}
#[cfg(feature = "r0-backend")]
pub use __r0::*;

#[cfg(feature = "sp1-backend")]
mod __sp1 {
    pub const TLS_ELF: &[u8] = include_bytes!("../tls-sp1/elf/tls.elf");
}

#[cfg(feature = "sp1-backend")]
pub use __sp1::*;
