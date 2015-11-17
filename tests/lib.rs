extern crate libracerd;
extern crate json_request;
extern crate rustc_serialize;
mod util;

mod http {
    use util::http::{self, UrlBuilder};

    use rustc_serialize::json::{Json};
    use json_request::{request_str, Method};

    #[test]
    fn find_definition() {
        http::with_server(|server| {
            // Build request args
            let url = server.url("/find_definition");
            let request_obj = stringify!({
                "buffers": [{
                    "file_path": "src.rs",
                    "contents": "fn foo() {}\nfn bar() {}\nfn main() {\nfoo();\n}"
                }],
                "file_path": "src.rs",
                "line": 4,
                "column": 3
            });

            // Make request
            let res = request_str(Method::Post, &url[..], Some(request_obj)).unwrap().unwrap();

            // Build actual/expected objects
            let actual = Json::from_str(&res[..]).unwrap();
            let expected = Json::from_str(stringify!({
                "text": "foo",
                "line": 1,
                "column": 3,
                "file_path": "src.rs"
            })).unwrap();

            // They should be equal
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn ping_pong() {
        http::with_server(|server| {
            let url = server.url("/ping");
            let res = request_str(Method::Get, &url[..], None).unwrap().unwrap();
            let actual = Json::from_str(&res[..]).unwrap();

            let expected = Json::from_str(stringify!({
                "pong": true
            })).unwrap();

            assert_eq!(actual, expected);
        });
    }
}
