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

    pub fn program(&mut self) -> &String {
        &self.program
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program;
    }

    pub fn version(&mut self) -> &String {
        &self.version
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn parse(&self) {}
}
