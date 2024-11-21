mod __sp1 {
    pub const TLS_ELF: &[u8] = include_bytes!("../tls-sp1/elf/tls.elf");
}

pub use __sp1::*;
