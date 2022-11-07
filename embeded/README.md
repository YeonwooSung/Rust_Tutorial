# Rust for embeded programming

As you might know, there is a famous github repo called [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) which is a template for building embedded applications in Rust.

## Quickstart with cortex-m-quickstart

### Dependencies

As you could see in the readme of the repo, you need to install the following dependencies:

1. Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. rustup default beta

2. The cargo generate subcommand.

```bash
cargo install cargo-generate
```

3. rust-std components (pre-compiled core crate) for the ARM Cortex-M targets. Run:

```bash
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```
