// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

pub use cmdargs_macros::*;

#[derive(Clone)]
pub struct Parser {
    pub program: String,
    pub version: String,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            program: String::new(),
            version: String::new(),
        }
    }

    pub fn program(mut self, program: String) -> Self {
        self.program = program;
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    pub fn parse(&self) {}
}
