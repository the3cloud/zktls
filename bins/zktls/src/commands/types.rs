use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum TargetChain {
    Evm,
    Solana,
    Sui,
    Aptos,
    Ton,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Prover {
    #[cfg(feature = "sp1-backend")]
    Sp1,
    #[cfg(feature = "r0-backend")]
    R0,
}
