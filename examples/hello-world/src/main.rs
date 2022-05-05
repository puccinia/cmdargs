// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

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
