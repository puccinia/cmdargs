// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

pub use cmdargs_macros::*;

use std::process;

#[derive(Clone)]
pub struct Parser {
    pub program: &'static str,
    pub version: &'static str,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            program: "",
            version: "",
        }
    }

    pub fn program(&mut self) -> &str {
        &self.program
    }

    pub fn set_program(&mut self, program: &'static str) {
        self.program = program;
    }

    pub fn version(&mut self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: &'static str) {
        self.version = version;
    }

    pub fn print_version(&self) {
        println!("{} v{}", self.program, self.version);
        process::exit(0);
    }

    pub fn parse(&self) {}
}
