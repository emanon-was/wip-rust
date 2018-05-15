use std::io;
use std::fs;
use std::path::PathBuf;

pub fn abspath(relative_path: &str) -> io::Result<PathBuf> {
    let path = PathBuf::from(relative_path);
    return fs::canonicalize(path);
}
