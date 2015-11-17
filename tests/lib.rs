extern crate libracerd;
extern crate json_request;
extern crate rustc_serialize;
mod util;

mod http {
    use util::http::{self, UrlBuilder};

    use rustc_serialize::json::{Json};
    use json_request::{request_str, Method};
    use libracerd::util::fs::TmpFile;

    /// Checks that /find_definition works within a single buffer
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

    /// Checks that /find_definition correctly resolves cross file definitions when the buffers have
    /// not been written to disk.
    #[test]
    fn find_definition_multiple_dirty_buffers() {
        // Build request args
        let request_obj = stringify!({
            "buffers": [{
                "file_path": "src.rs",
                "contents": "mod src2;\nuse src2::{foo,myfn};\nfn main() {\n    myfn();\n}\n"
            }, {
                "file_path": "src2.rs",
                "contents": "\npub fn myfn()\npub fn foo() {}\n"
            }],
            "file_path": "src.rs",
            "line": 4,
            "column": 7
        });

        // Create an *empty* temporary file. The request contains unsaved file contents. For rust
        // module inclusions to work, the compiler checks certain locations on the file system where
        // a module file could be located. It simply doesn't work with unnamed files.
        let _f = TmpFile::with_name("src2.rs", "");

        http::with_server(|server| {
            // Make request
            let url = server.url("/find_definition");
            let res = request_str(Method::Post, &url[..], Some(request_obj)).unwrap().unwrap();

            // Build actual/expected objects
            let actual = Json::from_str(&res[..]).unwrap();
            let expected = Json::from_str(stringify!({
                "text": "myfn",
                "line": 2,
                "column": 7,
                "file_path": "src2.rs"
            })).unwrap();

            // They should be equal
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn find_definition_in_std_library() {
        // Build request args
        let request_obj = stringify!({
            "buffers": [{
                "file_path": "src.rs",
                "contents": "use std::path::Path;\nfn main() {\nlet p = &Path::new(\"arst\")\n}\n"
            }],
            "file_path": "src.rs",
            "line": 3,
            "column": 16
        });

        http::with_server(|server| {
            // Make request
            let url = server.url("/find_definition");
            let res = request_str(Method::Post, &url[..], Some(request_obj)).unwrap().unwrap();

            // Build actual/expected objects
            let actual = Json::from_str(&res[..]).unwrap();

            // We don't know exactly how the result is going to look in this case.
            let obj = actual.as_object().unwrap();

            // Check that `file_path` ends_with "path.rs"
            let found_path = obj.get("file_path").unwrap().as_string().unwrap();
            assert!(found_path.ends_with("path.rs"));

            // Check that we found a thing called "new"
            let found_text = obj.get("text").unwrap().as_string().unwrap();
            assert_eq!(found_text, "new");
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
