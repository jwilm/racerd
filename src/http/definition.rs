use iron::prelude::*;
use iron::status;

use rustc_serialize::json;

use engine::{SemanticEngine, Racer, Definition, Context, CursorPosition};

/// Given a location, return where the identifier is defined
pub fn find(req: &mut Request) -> IronResult<Response> {

    // Parse the request
    let fdr = match req.get::<::bodyparser::Struct<FindDefinitionRequest>>() {
        Ok(Some(s)) => {
            s
        },
        Ok(None) => {
            return Ok(Response::with(status::BadRequest))
        },
        Err(err) => {
            return Err(IronError::new(err, status::InternalServerError))
        }
    };

    // Delegate to active semantic engine
    let racer = Racer;
    match racer.find_definition(&fdr.context()) {
        // Success, found the definition
        Ok(Some(definition)) => {
            let res = FindDefinitionResponse::from(definition);
            Ok(Response::with((status::Ok, json::encode(&res).unwrap())))
        },

        // Everything went ok, but the definition was not found.
        Ok(None) => Ok(Response::with(status::NoContent)),

        // Error occurred while searching for the definition
        Err(err) => {
            Err(IronError::new(err, status::InternalServerError))
        }
    }
}

impl From<Definition> for FindDefinitionResponse {
    fn from(def: Definition) -> FindDefinitionResponse {
        FindDefinitionResponse {
            file_path: def.file_path,
            column: def.position.col,
            line: def.position.line,
            text: def.text,
        }
    }
}

#[derive(Debug, RustcDecodable, Clone)]
struct FindDefinitionRequest {
    pub file_contents: String,
    pub file_path: String,
    pub column: usize,
    pub line: usize,
}

impl FindDefinitionRequest {
    pub fn context<'a>(&'a self) -> Context<'a> {
        let cursor = CursorPosition { line: self.line, col: self.column };
        Context::new(&self.file_contents[..], cursor, &self.file_path[..])
    }
}

#[test]
fn find_definition_request_from_json() {
    let s = stringify!({
        "file_path": "src.rs",
        "file_contents": "fn foo() {}\nfn bar() {}\nfn main() {\nfoo();\n}",
        "line": 4,
        "column": 3
    });

    let req = json::decode::<FindDefinitionRequest>(s).unwrap();
    assert_eq!(req.file_path, "src.rs");
    assert_eq!(req.line, 4);
    assert_eq!(req.column, 3);
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct FindDefinitionResponse {
    pub file_path: String,
    pub column: usize,
    pub line: usize,
    pub text: String,
}

// #[cfg(test)]
// mod tests {
//     use ::{Config, http};
// 
//     #[test]
//     fn self_contained() {
//         unimplemented!();
//     }
// 
//     #[test]
//     fn multiple_files() {
//         unimplemented!();
//     }
// 
//     #[test]
//     fn in_std() {
//         unimplemented!();
//     }
// }
