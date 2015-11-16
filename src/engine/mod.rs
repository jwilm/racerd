//! Engines provide source analysis for rust code
use std::path::Path;

/// This module's Error and Result types
mod error;
pub use self::error::{Error, Result};

/// Provide completions, definitions, and analysis of rust source code
pub trait SemanticEngine {
    /// Find the definition for the item under the cursor
    fn find_definition(&self, context: &Context) -> Result<Option<Definition>>;

    // Get a list of completions for the item under the cursor
    // fn complete(loc: &FileLocation) -> io::Result<Vec<Completion>>;
}

/// Source file and type information for a found definition
#[derive(Debug, RustcEncodable)]
pub struct Definition {
    pub position: CursorPosition,
    pub text: String,
    pub text_context: String,
    pub dtype: String,
    pub file_path: String,
}

/// Context for a given operation.
///
/// All operations require a buffer holding the contents of a file, the file's absolute path, and a
/// cursor position to fully specify the request. This object holds all of those items.
#[derive(Debug, RustcEncodable)]
pub struct Context<'a> {
    pub contents: &'a str,
    pub cursor: CursorPosition,
    pub file_path: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(contents: &'a str, position: CursorPosition, file_path: &'a str) -> Context<'a> {
        Context {
            contents: contents,
            cursor: position,
            file_path: file_path,
        }
    }

    pub fn path(&'a self) -> &'a Path {
        &Path::new(self.file_path)
    }
}

/// Position of the cursor in a text file
///
/// Similar to a point, it has two coordinates `line` and `col`.
#[derive(Debug, RustcEncodable)]
pub struct CursorPosition {
    pub line: usize,
    pub col: usize,
}

pub mod racer;
pub use self::racer::Racer;

