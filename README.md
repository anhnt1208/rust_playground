# Table of Contents

- [Table of Contents](#table-of-contents)
- [Introduction](#introduction)
- [Getting Started](#getting-started)
  - [Local Installation](#local-installation)
  - [Dockerize Rust](#dockerize-rust)
  - [Recommended VS Code extensions](#recommended-vs-code-extensions)
- [Reference](#reference)

# Introduction

Rust playground

# Getting Started

## Local Installation

Using `rushup` for managing Rust versions and associated tools.
Rust using `rustup` will also install `cargo`.

```bash
# Install rustup-init on MacOS https://formulae.brew.sh/formula/rustup-init
> brew install rustup-init
 # Init Rust
> rustup-init
# Check current version and update if needed
> rustc update
> rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)
```

## Dockerize Rust

```bash
$ docker build -f Dockerfile.local -t rust_playground .
$ docker run -it --rm --name rust_playground rust_playground
```

## Recommended VS Code extensions

- rust-analyzer: cornerstone extension for Rust developers
- crates: Keeping crates up-to-date can be complicated
- CodeLLDB: LLDB is a powerful debugger that supports Rust. It provides extended debugging capabilities compared to the default debugger

# Reference

- [[Rust cheat sheet](https://cheats.rs/)]
- [Easy Rust](https://dhghomon.github.io/easy_rust/Chapter_1.html)
- [Tour of Rust](https://tourofrust.com/index.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
