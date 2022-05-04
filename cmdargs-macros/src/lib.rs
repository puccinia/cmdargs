// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

use proc_macro::TokenStream;

#[proc_macro]
pub fn parser_from_str(_input: TokenStream) -> TokenStream {
    let program = r#"env!("CARGO_BIN_NAME")"#;
    let version = r#"env!("CARGO_PKG_VERSION")"#;

    let output = format!(
        r#"{{
            let mut parser = cmdargs::Parser::new();
            parser.set_program({});
            parser.set_version({});
            parser
        }}"#,
        program, version,
    );

    output.parse().unwrap()
}
