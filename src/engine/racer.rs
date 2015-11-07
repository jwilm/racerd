use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

use types::{Definition, Context, CursorPosition};

use engine::SemanticEngine;

pub struct Racer;

use super::Result;

fn read_file(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

impl SemanticEngine for Racer {
    fn lang() -> &'static str {
        "rust"
    }

    fn find_definition(ctx: &Context) -> Result<Option<Definition>> {
        let path = ctx.path();
        println!("making coords_to_point");
        let pos = ::racer::scopes::coords_to_point(ctx.contents, ctx.cursor.line, ctx.cursor.col);

        println!("making session");
        let session = ::racer::core::Session::from_path(path, path);

        println!("running find_definition: {:?}, {:?}, {:?}, {:?}", ctx.contents, path, pos, session);
        // TODO catch_panic: apparently this can panic! in a string operation. Something about pos
        // not landing on a character boundary.
        Ok(match ::racer::core::find_definition(ctx.contents, path, pos, &session) {
            Some(m) => {
                println!("got some");
                // TODO modify racer Match to return line, col. For now, read the file it found a
                // match in to translate the Match position into line, col.
                let (line, col) = {
                    let found_file = try!(read_file(m.filepath.as_path()));
                    // TODO This can panic if the position is out of bounds :(
                    ::racer::scopes::point_to_coords(&found_file[..], m.point)
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
    use types::{Context, CursorPosition};
    use super::*;
    use engine::SemanticEngine;
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::convert::From;
    use std::thread;
    use std::io::Write;

    struct TmpFile {
        path_buf: PathBuf
    }

    impl TmpFile {
        pub fn new(contents: &str) -> TmpFile {
            let tmp = TmpFile {
                path_buf: TmpFile::mktemp()
            };

            tmp.write_contents(contents);
            tmp
        }

        pub fn with_name(name: &str, contents: &str) -> TmpFile {
            let tmp = TmpFile {
                path_buf: PathBuf::from(name)
            };

            tmp.write_contents(contents);
            tmp
        }

        fn write_contents(&self, contents: &str) {
            let mut f = File::create(self.path()).unwrap();
            f.write_all(contents.as_bytes()).unwrap();
            f.flush().unwrap();
        }

        /// Make path for tmpfile. Stole this from racer's tests.
        fn mktemp() -> PathBuf {
            let thread = thread::current();
            let taskname = thread.name().unwrap();
            let s = taskname.replace("::", "_");
            let mut p = "tmpfile.".to_string();
            p.push_str(&s[..]);
            PathBuf::from(p)
        }

        pub fn path<'a>(&'a self) -> &'a Path {
            self.path_buf.as_path()
        }
    }

    impl Drop for TmpFile {
        fn drop(&mut self) {
            fs::remove_file(self.path_buf.as_path()).unwrap();
        }
    }

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
        let def = Racer::find_definition(&ctx).unwrap().unwrap();

        assert_eq!(def.text, "myfn");
    }
}
