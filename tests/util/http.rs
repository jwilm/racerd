use std::ops::Deref;
use std::fs::File;
use std::io::Write;

use libracerd::Config;
use libracerd::engine::{Racer, SemanticEngine};

/// Smart pointer for libracerd http server.
///
/// TestServer automatically closes the underlying server when going out of scope.
pub struct TestServer {
    inner: ::libracerd::http::Server
}

impl TestServer {
    pub fn new(secret_file: Option<String>) -> TestServer {
        let engine = Racer::new();
        let config = Config {
            port: 0,
            secret_file: secret_file,
            print_http_logs: true,
            rust_src_path: None,
        };

        engine.initialize(&config).unwrap();

        TestServer {
            inner: ::libracerd::http::serve(&config, engine).unwrap()
        }
    }
}

impl Deref for TestServer {
    type Target = ::libracerd::http::Server;
    fn deref(&self) -> &::libracerd::http::Server {
        &self.inner
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.inner.close().unwrap();
    }
}

pub trait UrlBuilder {
    /// Given a /url/path, return a full http URL.
    fn url(&self, path: &str) -> String;
}

impl UrlBuilder for TestServer {
    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr(), path)
    }
}

pub fn with_server<F>(mut func: F) where F: FnMut(&TestServer) -> () {
    func(&TestServer::new(None));
}

pub fn with_hmac_server<F>(secret: &str, mut func: F) where F: FnMut(&TestServer) -> () {
    // Make a temp file unique to this test
    let thread = ::std::thread::current();
    let taskname = thread.name().unwrap();
    let s = taskname.replace("::", "_");
    let mut p = "secretfile.".to_string();
    p.push_str(&s[..]);


    {
        let mut f = File::create(&p[..]).unwrap();
        f.write_all(secret.as_bytes()).unwrap();
        f.flush().unwrap();
    }

    func(&TestServer::new(Some(p)));
}

#[test]
fn server_url_builder() {
    with_server(|server| {
        let url = server.url("/definition");
        assert!(url.starts_with("http://"));
        assert!(url.ends_with("/definition"));
    });
}
