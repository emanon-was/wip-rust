use std::result;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;

use os::path;

pub type Result<T> = result::Result<T, Message>;
pub type Message = String;

#[derive(Debug)]
pub struct ReadFile {
    path: String,
    body: String,
}

impl ReadFile {
    pub fn new(path: &str, body: &str) -> ReadFile {
        ReadFile {
            path: path.to_string(),
            body: body.to_string(),
        }
    }
}

pub fn read(path: &str) -> Result<ReadFile> {
    let f = path::abspath(path);
    match f {
        Err(err) => Err(format!("{:}", err)),
        Ok(pathbuf) => read_file(pathbuf),
    }
}

pub fn read_file(pathbuf: PathBuf) -> Result<ReadFile> {
    if !pathbuf.is_file() {
        return Err(String::from("No such file"));
    }
    let path = pathbuf.to_str().unwrap();
    let file = File::open(path);
    match file {
        Err(err) => Err(format!("{:}", err)),
        Ok(mut f) => {
            let mut s = String::new();
            &f.read_to_string(&mut s);
            return Ok(ReadFile::new(path, &s));
        }
    }
}
