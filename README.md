# codspeed-guide-rust

A Rust benchmarking example using [Divan](https://github.com/nvzqz/divan) and [CodSpeed](https://codspeed.io), following the official [CodSpeed guide for Rust](https://codspeed.io/docs/guides/how-to-benchmark-rust-with-divan).

## What this repo covers

- Setting up Divan as a benchmark harness in a Rust project
- Writing benchmarks: parametric, async, type-generic, and setup/measure separation
- Integrating CodSpeed CI to track performance regressions and improvements over time

## Stack

- **Language**: Rust (edition 2024)
- **Benchmark harness**: [`codspeed-divan-compat`](https://crates.io/crates/codspeed-divan-compat) (drop-in Divan replacement for CodSpeed)
- **Performance monitoring**: [CodSpeed](https://codspeed.io)

## Project structure

```
.
├── benches/
│   ├── fibonacci.rs   # fib, JSON parsing, vec search, async, type-generic benchmarks
│   └── sorting.rs     # bubble_sort and merge_sort benchmarks
├── src/
│   ├── lib.rs
│   ├── fibonacci.rs   # fib, generate_large_json
│   └── sorting.rs     # bubble_sort, merge_sort
├── .github/
│   └── workflows/
│       └── codspeed.yml
└── Cargo.toml
```

## Run benchmarks locally

```bash
# Standard Divan output
cargo bench

# CodSpeed instrumented run (simulation mode outside CI)
cargo codspeed run
```

## CI

Benchmarks run automatically on every push to `main` and on every pull request via GitHub Actions. CodSpeed compares each PR against the `main` baseline and reports regressions or improvements directly in the PR.

## PR demo — CodSpeed workflow in action

Three pull requests demonstrate the three result types CodSpeed can surface:

| PR | Branch | Scenario | What CodSpeed reports |
|---|---|---|---|
| [#1 feat: add merge_sort](https://github.com/s3bc40/codspeed-guide-rust/pull/1) | `feat/merge-sort` | New feature | New benchmarks, no baseline comparison |
| [#2 fix: bubble_sort refactor](https://github.com/s3bc40/codspeed-guide-rust/pull/2) | `fix/bubble-sort-regression` | Regression | Performance degradation on `bench_bubble_sort` |
| [#3 perf: iterative fibonacci](https://github.com/s3bc40/codspeed-guide-rust/pull/3) | `perf/memoized-fib` | Improvement | Dramatic speedup on `bench_fib` and `bench_fib_args` |
