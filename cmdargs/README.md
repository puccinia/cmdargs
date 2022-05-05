# cmdargs

Command line argument parser for Rust.

[![Crates.io](https://img.shields.io/crates/v/cmdargs)](https://crates.io/crates/cmdargs)
[![Crates.io](https://img.shields.io/crates/d/cmdargs)](https://crates.io/crates/cmdargs)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cmdargs)
[![License](https://img.shields.io/badge/license-Unlicense-blue)](https://unlicense.org)
[![Contributors](https://img.shields.io/github/contributors/puccinia/cmdargs)](https://github.com/puccinia/cmdargs/graphs/contributors)

**NOTE: this library is under development and "not production ready".**

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
cmdargs = "0.1.0"
```

Example:

```rust
use cmdargs;

fn main() {
    let parser = cmdargs::parser_from_str! {"
        Usage: hello-world [options]

        Options:
          -a, --all        Print Hello World
          -H, --hello      Print Hello
          -W, --world      Print World

          -h, --help       Print help information
          -V, --version    Print version information
    "};
    parser.parse();

    println!("Hello World!");
}
```
