use std::ops::Deref;

use libracerd::Config;

/// Smart pointer for libracerd http server.
///
/// TestServer automatically closes the underlying server when going out of scope.
pub struct TestServer {
    inner: ::libracerd::http::Server
}

impl TestServer {
    pub fn new() -> TestServer {
        TestServer {
            inner: ::libracerd::http::serve(&Config {
                port: 0,
                secret_file: "/tmp/secret".to_string(),
                print_http_logs: true
            }).unwrap()
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
    func(&TestServer::new());
}

#[test]
fn server_url_builder() {
    with_server(|server| {
        let url = server.url("/definition");
        assert!(url.starts_with("http://"));
        assert!(url.ends_with("/definition"));
    });
}
