//! SemanticEngine implementation for [the racer library](https://github.com/phildawes/racer)
//!
use engine::{SemanticEngine, Definition, Context, CursorPosition, Completion, Error};

use racer::core::{Session, FileCache, IndexedSource};

use std::path::Path;
use std::sync::Mutex;
use std::rc::Rc;

use regex::Regex;

pub struct Racer {
    cache: Mutex<FileCache>
}

impl Racer {
    pub fn new() -> Racer {
        Racer {
            cache: Mutex::new(FileCache::new()),
        }
    }

    fn build_racer_args<'a, 'b, 'c>(&'a self,
                                    ctx: &'b Context,
                                    session: &'c Session<'a>)
                                    -> (Rc<IndexedSource>, usize, &'b Path)
    {
        let path = ctx.query_path();

        for buffer in &ctx.buffers {
            session.cache_file_contents(buffer.path(), &*buffer.contents)
        }

        let file = session.load_file(path);
        let pos = file.coords_to_point(ctx.query_cursor.line,
                                       ctx.query_cursor.col).unwrap();

        (file, pos, path)
    }
}

// FIXME: These impls are sort of a lie, since we could theoretically hold a
// `Rc<IndexedSource>` for too long.
//
// They're needed by the http middleware, and our usage turns out to be fine,
// since we're only holding Rc's while we hold the cache lock, but something
// more elegant (like making racer use `Arc`'s) should be done instead.
unsafe impl Sync for Racer {}
unsafe impl Send for Racer {}

use ::Config;
use super::Result;

impl SemanticEngine for Racer {
    fn initialize(&self, config: &Config) -> Result<()> {
        if let Some(ref src_path) = config.rust_src_path {
            ::std::env::set_var("RUST_SRC_PATH", src_path);
        }

        Ok(())
    }

    fn find_definition(&self, ctx: &Context) -> Result<Option<Definition>> {
        let cache = match self.cache.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let session = Session::from_path(&cache, ctx.query_path(), ctx.query_path());
        let (file, pos, path) = self.build_racer_args(ctx, &session);

        // TODO catch_panic: apparently this can panic! in a string operation. Something about pos
        // not landing on a character boundary.
        Ok(match ::racer::core::find_definition(&file.code, path, pos, &session) {
            Some(m) => {
                // TODO modify racer Match to return line, col. For now, read the file it found a
                // match in to translate the Match position into line, col.
                let (line, col) = {
                    let file = session.load_file(&m.filepath);
                    try!(file.point_to_coords(m.point).ok_or(Error::Racer))
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
                    docs: m.docs.clone(),
                })
            },
            None => None
        })
    }

    fn list_completions(&self, ctx: &Context) -> Result<Option<Vec<Completion>>> {
        let cache = match self.cache.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let session = Session::from_path(&cache, ctx.query_path(), ctx.query_path());
        let (file, pos, path) = self.build_racer_args(ctx, &session);

        let matches = ::racer::core::complete_from_file(&file.code, path, pos, &session);

        let completions = matches.filter_map(|m| {
            let (line, col) = {
                let file = session.load_file(&m.filepath);
                match file.point_to_coords(m.point) {
                    Some((line, col)) => (line, col),
                    None => return None,
                }
            };

            Some(Completion {
                position: CursorPosition {
                    line: line,
                    col: col
                },
                text: m.matchstr,
                context: collapse_whitespace(&m.contextstr),
                kind: format!("{:?}", m.mtype),
                file_path: m.filepath.to_str().unwrap().to_string()
            })
        }).collect::<Vec<_>>();

        if completions.len() != 0 {
            Ok(Some(completions))
        } else {
            Ok(None)
        }
    }
}

pub fn collapse_whitespace(text: &str) -> String {
  Regex::new(r"\s+").unwrap().replace_all(text, " ")
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
            /// myfn docs
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

        let ctx = Context::new(buffers, CursorPosition { line: 5, col: 17 }, "src.rs");
        let racer = Racer::new();
        let def = racer.find_definition(&ctx).unwrap().unwrap();

        assert_eq!(def.text, "myfn");
        assert_eq!(def.docs, "myfn docs");
    }

    #[test]
    #[allow(unused_variables)]
    fn find_completion() {
        let src = "
            mod mymod {
                /// myfn is a thing
                pub fn myfn<T>(reader: T) where T: ::std::io::Read {}
            }

            fn main() {
                mymod::my
            }";

        let buffers = vec![Buffer {
            contents: src.to_string(),
            file_path: "src.rs".to_string()
        }];

        let ctx = Context::new(buffers, CursorPosition { line: 8, col: 25 }, "src.rs");
        let racer = Racer::new();

        let completions = racer.list_completions(&ctx).unwrap().unwrap();
        assert_eq!(completions.len(), 1);

        let completion = &completions[0];
        assert_eq!(completion.text, "myfn");
        assert_eq!(completion.position.line, 4);
        assert_eq!(completion.file_path, "src.rs");
    }

    #[test]
    fn find_completion_collapsing_whitespace() {
        let src = "
            struct foo {}

            impl foo {
              fn format( &self )
                -> u32 {
              }
            }

            fn main() {
              let x = foo{};
              x.
            }";

        let buffers = vec![Buffer {
            contents: src.to_string(),
            file_path: "src.rs".to_string()
        }];

        let ctx = Context::new(buffers, CursorPosition { line: 12, col: 16 }, "src.rs");
        let racer = Racer::new();

        let completions = racer.list_completions(&ctx).unwrap().unwrap();
        assert_eq!(completions.len(), 1);

        let completion = &completions[0];
        assert_eq!(completion.context, "fn format( &self ) -> u32");
    }

    #[test]
    fn collapse_whitespace_test() {
        assert_eq!(collapse_whitespace("foo  foo"), "foo foo");
        assert_eq!(collapse_whitespace("  "), " ");
        assert_eq!(collapse_whitespace("\n\t  \n"), " ");
        assert_eq!(collapse_whitespace("foo\nbar"), "foo bar");
        assert_eq!(collapse_whitespace("fn foo( &self )\n   -> u32"),
                   "fn foo( &self ) -> u32");
    }
}
