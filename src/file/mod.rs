use std;
use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct ReadFile {
    path: String,
    body: String
}

impl ReadFile {
    pub fn new(path: &str, body: &str) -> ReadFile {
        ReadFile {
            path: String::from(path),
            body: String::from(body)
        }
    }
}

// derive(Debug)
// impl std::fmt::Debug for ReadFile {
//     fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f,"ReadFile{path:{:?},body:{:?}}",&self.path,&self.body)
//     }
// }

pub fn abspath(relative_path: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let path = PathBuf::from(relative_path);
    return fs::canonicalize(path);
}

pub fn read_file(path: &str) -> Result<ReadFile,String> {
    let f = abspath(path);
    match f {
        Err(err) => Err(format!("{:}",err)),
        Ok(pathbuf) => read_path(pathbuf)
    }
}

fn read_path(pathbuf: PathBuf) -> Result<ReadFile,String> {
    if !pathbuf.is_file() {
        return Err(String::from("No such file"));
    } else {
        return read_body(pathbuf)
    }
}

fn read_body(pathbuf: PathBuf) -> Result<ReadFile,String> {
    let path = pathbuf.to_str().unwrap();
    let file = File::open(path);
    match file {
        Err(err) => Err(format!("{:}",err)),
        Ok(mut f) => {
            let mut s = String::new();
            &f.read_to_string(&mut s);
            return Ok(ReadFile::new(path,&s));
        }
    }
}
