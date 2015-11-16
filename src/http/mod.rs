//! http server for semantic engines
//!
//! Provide semantic completion, definition look-up, and compilation warning/error JSON API over
//! HTTP. Endpoints currently assume the `Racer` engine.
//!
//! # Definition lookup
//!
//! `POST /find_definition`
//!
//! ## Request
//!
//! ```json
//! {
//!     "file_path": "src.rs",
//!     "file_contents": "<some rust code>",
//!     "line": 4,
//!     "column": 3
//! }
//! ```
//!
//! ## Response
//!
//! ```json
//! {
//!     "file_path": "maybe_other_src.rs",
//!     "text": "fn foo() {}",
//!     "line": 4,
//!     "column": 3
//! }
//! ```
//!
//! # Planned features
//! ✓ Definition lookup
//!
//! ☐ HMAC Auth for all endpoints
//!
//! ☐ Code completions
//!
//! ☐ File compilation

use iron::{Iron, Chain};

use ::Config;

mod definition;
mod file;
mod completion;
mod ping;

/// Errors occurring in the http module
#[derive(Debug)]
pub enum Error {
    /// Error occurred in underlying http server lib
    HttpServer(::hyper::Error),
    // Error occurred in http framework layer
    // HttpApp(::iron::IronError),
}

impl From<::hyper::Error> for Error {
    fn from(err: ::hyper::Error) -> Error {
        Error::HttpServer(err)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

/// Start the http server using the given configuration
///
/// `serve` is non-blocking.
///
/// # Example
///
/// ```no_run
/// # use libracerd::{Config};
/// let mut cfg = Config::new();
/// cfg.port = 3000;
///
/// let mut server = ::libracerd::http::serve(&cfg).unwrap();
/// // ... later
/// server.close().unwrap();
/// ```
///
pub fn serve(config: &Config) -> Result<Server> {
    use persistent::Read;
    use logger::Logger;

    let mut chain = Chain::new(router!(
        post "/parse_file"       => file::parse,
        post "/find_definition"  => definition::find,
        post "/list_completions" => completion::list,
        get  "/ping"             => ping::pong));

    // Logging middleware
    let (log_before, log_after) = Logger::new(None);

    // log_before must be first middleware in before chain
    if config.print_http_logs {
        chain.link_before(log_before);
    }

    // Body parser middlerware
    chain.link_before(Read::<::bodyparser::MaxBodyLength>::one(1024 * 1024 * 10));

    // log_after must be last middleware in after chain
    if config.print_http_logs {
        chain.link_after(log_after);
    }

    let app = Iron::new(chain);
    let host = format!("0.0.0.0:{}", config.port);

    Ok(Server {
        inner: try!(app.http(&host[..]))
    })
}

/// Wrapper type with information and control of the underlying HTTP server
///
/// This type can only be created via the [`serve`](fn.serve.html) function.
#[derive(Debug)]
pub struct Server {
    inner: ::hyper::server::Listening,
}

impl Server {
    /// Stop accepting connections
    pub fn close(&mut self) -> Result<()> {
        Ok(try!(self.inner.close()))
    }

    /// Get listening address of server (eg. "127.0.0.1:59369")
    ///
    /// # Example
    /// ```no_run
    /// let mut config = ::libracerd::Config::new();
    /// config.port = 3000;
    ///
    /// let server = ::libracerd::http::serve(&config).unwrap();
    ///
    /// assert_eq!(server.addr(), "0.0.0.0:3000");
    /// ```
    pub fn addr(&self) -> String {
        format!("{}", self.inner.socket)
    }
}
