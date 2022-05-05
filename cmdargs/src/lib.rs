// cmdargs - Command line argument parser for Rust
//
// Copyright waived 2022, Everaldo Canuto <everaldo.canuto@gmail.com>
//
// The contents of this file is free and unencumbered software released into the
// public domain. For more information, please refer to <http://unlicense.org/>

pub use cmdargs_macros::*;

use std::{env, process};

#[derive(Clone)]
pub struct Parser {
    pub program: &'static str,
    pub version: &'static str,
    pub usage: String,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            program: "",
            version: "",
            usage: String::new(),
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

    pub fn usage(&mut self) -> &String {
        &self.usage
    }

    pub fn set_usage(&mut self, usage: String) {
        self.usage = usage;
    }

    pub fn print_version(&self) {
        println!("{} v{}", self.program, self.version);
        process::exit(0);
    }

    pub fn print_usage(&self) {
        if self.usage.is_empty() {
            println!("Usage: {} [options]", self.program);
        } else {
            println!("{}", self.usage);
        }
        process::exit(0);
    }

    pub fn parse_from(&self, args: impl Iterator<Item = String>) {
        for item in args {
            match item.as_str() {
                "-h" | "--help" => self.print_usage(),
                "-V" | "--version" => self.print_version(),
                _ => println!("{}", item),
            }
        }
    }
    pub fn parse(&self) {
        self.parse_from(env::args().skip(1));
    }
}
