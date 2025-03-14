# zkTLS

> ZkTLS: Acces anys web2 data in web3 world trustless.

## What is zkTLS

TLS is a widely used foundational internet protocol that ensures the security and integrity of data transmission. Additionally, TLS uses PKI to enable entity authentication. By combining zero-knowledge (zk) with TLS, we construct the zkTLS protocol. The zkTLS can prove the connection process of a TLS connection. This proof enables trustless access for Web3 to any endpoint that supports TLS.

## Dependencies

- [foundry](https://book.getfoundry.sh/getting-started/installation)
- [CUDA](https://developer.nvidia.com/cuda-downloads) (optional)
- openssl
- pkg-config
- risc0 backend
  - [risc0](https://dev.risczero.com/api/zkvm/install)
- sp1 backend
  - [sp1](https://docs.succinct.xyz/getting-started/install.html)
  - [CUDA Docker Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) (optional)

## Usage

1. Clone the repo

```bash
git clone https://github.com/the3cloud/zktls.git
```

2. Compile the project

```bash
cargo b && cargo t && cargo c
```

3. Run zktls in command listens

```bash
cargo run --bin zktls -- prove --input-request-file <path-to-request> --target-chain <target-chain> --mock
```

- target-chain: `evm`, `solana`

## Future Work

- Use `mbedtls` instead of `rustls`.
- Support batch proof submitted.

## Benchmark

see [benchmark.md](./benchmark.md)

## For development

This is a monorepo. It means all code are in a single repo. We don't use any other build system, just `cargo`. You can use `cargo build` and `cargo test` for all codebase, include contract code.

### Project Structure

- contracts: solidity contract. other chain contract in future.
- crates
  - recordable-tls: simulate TLS record, and record all messages.
  - contracts: contracts warp crate.
  - kms: store encrypted key.
  - listens: pull TLS request event.
