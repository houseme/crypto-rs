# Crypto-rs
A general solution for commonly used crypt in rust, collection of cryptography-related traits and algorithms.


[![Build](https://github.com/houseme/crypto-rs/workflows/Build/badge.svg)](https://github.com/houseme/crypto-rs/actions?query=workflow%3ABuild)
[![crates.io](https://img.shields.io/crates/v/crypto-rs.svg)](https://crates.io/crates/crypto-rs)
[![docs.rs](https://docs.rs/crypto-rs/badge.svg)](https://docs.rs/crypto-rs/)
[![License](https://img.shields.io/crates/l/crypto-rs)](LICENSE-APACHE)

This is a Rust implementation of the original [houseme/gocrypto](https://github.com/houseme/gocrypto), which is written in Go.



## Install

Add the following to your `Cargo.toml`:
```toml
[dependencies]
crypto-rs = "0.1"
```

## Quickstart

```rust

```

## Benchmarks


Run them yourself with `cargo bench`.

#### 1, Benchmarks were run on a MacBook Pro (16-inch, 2019) with a 2,4GHz i9 and 64 GB memory.

```
test bench_decompose ... bench:         651 ns/iter (+/- 251)
test bench_new       ... bench:     795,722 ns/iter (+/- 371,556)
test bench_next_id   ... bench:      36,652 ns/iter (+/- 1,105)
```

#### 2, Benchmarks were run on a MacBook Pro (15-inch, 2017) with a 2,8GHz i7 and 16 GB memory.

```
test bench_decompose ... bench:       1,066 ns/iter (+/- 132)
test bench_new       ... bench:     738,129 ns/iter (+/- 318,192)
test bench_next_id   ... bench:      37,390 ns/iter (+/- 499)
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
