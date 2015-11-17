//! SemanticEngine implementation for [the racer library](https://github.com/phildawes/racer)
use engine::{SemanticEngine, Definition, Context, CursorPosition};

pub struct Racer;

use super::Result;

impl SemanticEngine for Racer {
    fn find_definition(&self, ctx: &Context) -> Result<Option<Definition>> {
        let path = ctx.query_path();

        let session = ::racer::core::Session::from_path(path, path);

        for buffer in ctx.buffers {
            session.cache_file_contents(buffer.path(), &buffer.contents[..]);
        }

        let query_src = &session.load_file(path).src.code[..];

        let pos = ::racer::scopes::coords_to_point(query_src,
                                                   ctx.query_cursor.line,
                                                   ctx.query_cursor.col);

        // TODO catch_panic: apparently this can panic! in a string operation. Something about pos
        // not landing on a character boundary.
        Ok(match ::racer::core::find_definition(query_src, path, pos, &session) {
            Some(m) => {
                // TODO modify racer Match to return line, col. For now, read the file it found a
                // match in to translate the Match position into line, col.
                let (line, col) = {
                    let found_src = &session.load_file(m.filepath.as_path());
                    let found_src_str = &found_src.src.code[..];
                    // TODO This can panic if the position is out of bounds :(
                    ::racer::scopes::point_to_coords(found_src_str, m.point)
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
    use engine::{Context, CursorPosition, SemanticEngine, Buffer};

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

        let buffers = vec![Buffer {
            contents: src.to_string(),
            file_path: "src.rs".to_string()
        }];

        let ctx = Context::new(&buffers, CursorPosition { line: 5, col: 17 }, "src.rs");
        let racer = Racer;
        let def = racer.find_definition(&ctx).unwrap().unwrap();

        assert_eq!(def.text, "myfn");
    }
}
