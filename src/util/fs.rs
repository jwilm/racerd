use std::io::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::convert::From;
use std::thread;

pub struct TmpFile {
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
fn tmp_file_works() {
    fn exists(p: &Path) -> bool {
        match ::std::fs::metadata(p) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    use std::fs::{File};
    use std::path::Path;
    use std::io::Read;

    let path_str = "test.txt";
    let path = &Path::new(path_str);
    assert!(!exists(path));

    let contents = "hello, for a moment";

    {
        let file = TmpFile::with_name(path_str, contents);
        assert!(exists(path));

        let mut f = File::open(path_str).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        assert_eq!(s, contents);
    }

    assert!(!exists(path));
}
