libnode-membership
==================
[![Rust: nightly](https://img.shields.io/badge/Rust-nightly-blue.svg)](https://www.rust-lang.org) [![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE) [![Build Status](https://travis-ci.org/Fantom-foundation/libnode-membership.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libnode-membership)

This is a node membership library in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### CLI instructions

```bash
# Install Rust (nightly)
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Install clippy (Rust linter)
$ rustup component add clippy
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libnode-membership && cd libnode-membership
# Run tests
$ cargo test
# Format, build and test
$ cargo make
# Compile the documentation in `target/doc`
$ cargo doc
```
