extern crate docopt;
extern crate rustc_serialize;
extern crate libracerd;

use std::convert::Into;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
racerd - a JSON/HTTP layer on top of racer

Usage:
  racerd serve --secret-file=<path> [--port=<int>]
  racerd (-h | --help)
  racerd --version

Options:
  -h, --help                Show this message.
  -p, --port=<int>          Listen on this port [default: 3048].
  -s, --secret-file=<path>  Path to the HMAC secret file. File will be destroyed after being read.
  --version                 Print the version and exit.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: usize,
    flag_version: bool,
    flag_secret_file: String,
    cmd_serve: bool
}

impl Into<libracerd::Config> for Args {
    fn into(self) -> libracerd::Config {
        libracerd::Config {
            port: self.flag_port,
            secret_file: self.flag_secret_file
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // Print version and exit if --version was specified
    if args.flag_version {
        println!("racerd version {}", VERSION);
        ::std::process::exit(0);
    }

    libracerd::http::serve(args.into());
}
