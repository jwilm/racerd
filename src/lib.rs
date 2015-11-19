//! libracerd provides an http server and set of completion engines for consumption by rust
//! developer tools. The http server itself is a few JSON endpoints providing completion, definition
//! look-up, and compilation. The endpoints are backed by an object implementing `SemanticEngine`
//!
//! Documentation for the HTTP endpoints can be found in the http module header.
//!
//! This project's source code is [available on GitHub](https://github.com/jwilm/racerd).
extern crate rustc_serialize;

#[macro_use]
extern crate router;     // Iron routing handler
extern crate bodyparser; // Iron body parsing middleware
extern crate persistent; // Iron storage middleware
extern crate logger;     // Iron logging middleware
extern crate iron;       // http framework

extern crate hyper;      // Provides `Listening` type returned by Iron

#[macro_use]
extern crate log;        // log macros
extern crate racer;      // rust code analysis
extern crate crypto;     // crypto algorithms for HMAC auth middleware

extern crate rand;

pub mod util;
pub mod http;
pub mod engine;

/// Configuration flags and values
///
/// This object contains all switches the consumer has control of.
#[derive(Debug, Default)]
pub struct Config {
    pub port: u16,
    pub secret_file: String,
    pub print_http_logs: bool,
    pub rust_src_path: Option<String>
}

impl Config {
    /// Build a default config object
    pub fn new() -> Config {
        Default::default()
    }
}
