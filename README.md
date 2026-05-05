# codspeed-guide-rust

A minimal Rust benchmarking example using [Divan](https://github.com/nvzqz/divan) and [CodSpeed](https://codspeed.io), following the official [CodSpeed guide for Rust](https://codspeed.io/docs/guides/how-to-benchmark-rust-with-divan).

## What this repo covers

- Setting up Divan as a benchmark harness in a Rust project
- Writing a simple benchmark (`fib` function) to measure execution time
- Integrating CodSpeed to track performance over time in CI

## Stack

- **Language**: Rust (edition 2024)
- **Benchmark harness**: [Divan](https://crates.io/crates/divan)
- **Performance monitoring**: [CodSpeed](https://codspeed.io)

## Run benchmarks locally

```bash
cargo bench
```

## Project structure

```
.
├── benches/
│   └── fibonacci.rs   # Divan benchmark entry point
├── src/
│   └── main.rs
└── Cargo.toml
```
