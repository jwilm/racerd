//! SemanticEngine implementation for [the racer library](https://github.com/phildawes/racer)
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

use engine::{SemanticEngine, Definition, Context, CursorPosition};

pub struct Racer;

use super::Result;

fn read_file(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

impl SemanticEngine for Racer {
    fn find_definition(&self, ctx: &Context) -> Result<Option<Definition>> {
        let path = ctx.path();

        let pos = ::racer::scopes::coords_to_point(ctx.contents, ctx.cursor.line, ctx.cursor.col);
        let session = ::racer::core::Session::from_path(path, path);

        session.cache_file_contents(path, ctx.contents);

        // TODO catch_panic: apparently this can panic! in a string operation. Something about pos
        // not landing on a character boundary.
        Ok(match ::racer::core::find_definition(ctx.contents, path, pos, &session) {
            Some(m) => {
                // TODO modify racer Match to return line, col. For now, read the file it found a
                // match in to translate the Match position into line, col.
                let (line, col) = {
                    // TODO This can panic if the position is out of bounds :(
                    if m.filepath.as_path() == path {
                        ::racer::scopes::point_to_coords(ctx.contents, m.point)
                    } else {
                        let found_file = try!(read_file(m.filepath.as_path()));
                        ::racer::scopes::point_to_coords(&found_file[..], m.point)
                    }
                };

                let match_type = format!("{:?}", m.mtype);
                Some(Definition {
                    position: CursorPosition {
                        line: line,
                        col: col
                    },
                    dtype: match_type,
                    file_path: m.filepath.to_str().unwrap().to_string(),
                    text: m.matchstr.clone(),
                    text_context: m.contextstr.clone(),
                })
            },
            None => None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::{Context, CursorPosition, SemanticEngine};

    use ::util::fs::TmpFile;

    #[test]
    #[allow(unused_variables)]
    fn find_definition() {
        // FIXME this is just here for side effects.
        let src2 = TmpFile::with_name("src2.rs", "
            pub fn myfn() {}
            pub fn foo() {}
            ");

        let src = "
            use src2::*;
            mod src2;
            fn main() {
                myfn();
            }";

        // FIXME The source file being examined _must_ be written out to disk, or the racer
        // find_definition impl will panic. What is the point of passing in a buffer if it reads the
        // file anyway?
        let dummy = TmpFile::new(src);

        let ctx = Context::new(src, CursorPosition { line: 5, col: 17 }, dummy.path().to_str().unwrap());
        let racer = Racer;
        let def = racer.find_definition(&ctx).unwrap().unwrap();

        assert_eq!(def.text, "myfn");
    }
}
