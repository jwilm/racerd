use iron::prelude::*;
use iron::status;

use http::definition;
use rustc_serialize::json;

use engine::{SemanticEngine, Racer, Completion, Context, CursorPosition, Buffer};

/// Given a location, return a list of possible completions
pub fn list(req: &mut Request) -> IronResult<Response> {
    let lcr = match req.get::<::bodyparser::Struct<ListCompletionsRequest>>() {
        Ok(Some(s)) => {
            trace!("parsed ListCompletionsRequest");
            s
        },
        Ok(None) => {
            trace!("failed parsing ListCompletionsRequest");
            return Ok(Response::with(status::BadRequest))
        },
        Err(err) => {
            trace!("error while parsing ListCompletionsRequest");
            return Err(IronError::new(err, status::InternalServerError))
        }
    };

    let racer = Racer;
    match racer.list_completions(&lcr.context()) {
        // 200 OK; found the definition
        Ok(Some(completions)) => {
            trace!("got a match");
            let res = completions.into_iter().map(|c| CompletionResponse::from(c)).collect::<Vec<_>>();
            Ok(Response::with((status::Ok, json::encode(&res).unwrap())))
        },

        // 204 No Content; Everything went ok, but the definition was not found.
        Ok(None) => {
            trace!("did not find any match");
            Ok(Response::with(status::NoContent))
        },

        // 500 Internal Server Error; Error occurred while searching for the definition
        Err(err) => {
            trace!("encountered an error");
            Err(IronError::new(err, status::InternalServerError))
        }
    }
}

#[derive(Debug, RustcDecodable, Clone)]
struct ListCompletionsRequest {
    pub buffers: Vec<Buffer>,
    pub file_path: String,
    pub column: usize,
    pub line: usize,
}

impl ListCompletionsRequest {
    pub fn context(self) -> Context {
        let cursor = CursorPosition { line: self.line, col: self.column };
        Context::new(self.buffers, cursor, self.file_path)
    }
}

type ListCompletionsResponse = Vec<CompletionResponse>;

#[derive(Debug, RustcEncodable)]
struct CompletionResponse {
    text: String,
    context: String,
    kind: String,
    file_path: String,
    line: usize,
    column: usize
}

impl From<Completion> for CompletionResponse {
    fn from(c: Completion) -> CompletionResponse {
         CompletionResponse {
            text: c.text,
            context: c.context,
            kind: c.kind,
            file_path: c.file_path,
            line: c.position.line,
            column: c.position.col
         }
    }
}

