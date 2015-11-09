extern crate iron;
extern crate rustc_serialize;
#[macro_use]
extern crate router;
extern crate bodyparser;
extern crate racer;
extern crate crypto;


mod engine;
mod types;

#[derive(Debug)]
pub struct Config {
    pub port: usize,
    pub secret_file: String
}

pub mod http;
