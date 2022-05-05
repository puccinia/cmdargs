// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
pub fn parser_from_str(input: TokenStream) -> TokenStream {
    let program = r#"env!("CARGO_BIN_NAME")"#;
    let version = r#"env!("CARGO_PKG_VERSION")"#;

    let mut usage = String::new();

    for tt in input.into_iter() {
        match tt {
            TokenTree::Literal(value) => {
                usage = value.to_string();
                break;
            }
            _ => {}
        }
    }

    if !usage.is_empty() {
        usage = format!("parser.set_usage(String::from({}))", usage);
    }

    let output = format!(
        r#"{{
            let mut parser = cmdargs::Parser::new();
            parser.set_program({});
            parser.set_version({});
            {};
            parser
        }}"#,
        program, version, usage,
    );

    // eprintln!("{}", output);

    output.parse().unwrap()
}
