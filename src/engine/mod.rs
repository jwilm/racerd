use types::{Definition, Context};

/// This module's Error and Result types
mod error;
pub use self::error::{Error, Result};

/// A SemanticEngine provides completions, locations, and analysis of source code for a give
/// language.
trait SemanticEngine {
    /// The language this implementation is for
    fn lang() -> &'static str;

    /// Find the definition for the item under the cursor
    fn find_definition(context: &Context) -> Result<Option<Definition>>;

    // Get a list of completions for the item under the cursor
    // fn complete(loc: &FileLocation) -> io::Result<Vec<Completion>>;
}

// Reexport the Racer semantic engine
mod racer;
pub use self::racer::Racer;
