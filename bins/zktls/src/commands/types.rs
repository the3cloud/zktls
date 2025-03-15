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
    Sp1,
    R0,
}
