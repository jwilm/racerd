//! libracerd provides an http server and set of completion engines for consumption by rust
//! developer tools. The http server itself is a few JSON endpoints providing completion, definition
//! look-up, and compilation. The endpoints are backed by an object implementing `SemanticEngine`
//!
//! Documentation for the HTTP endpoints can be found in the http module header.
//!
//! This project's source code is [available on GitHub](https://github.com/jwilm/racerd).
extern crate iron;
extern crate rustc_serialize;
#[macro_use]
extern crate router;
extern crate bodyparser;
extern crate racer;
extern crate crypto;
extern crate hyper;
extern crate persistent;
extern crate logger;

mod util;
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
}

impl Config {
    /// Build a default config object
    pub fn new() -> Config {
        Default::default()
    }
}
