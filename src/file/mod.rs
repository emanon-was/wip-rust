use std;
use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct TextFile {
    path: String,
    body: String
}

impl TextFile {
    pub fn new(path: &str, body: &str) -> TextFile {
        TextFile {
            path: path.to_string(),
            body: body.to_string()
        }
    }
}

// derive(Debug)
// impl std::fmt::Debug for TextFile {
//     fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f,"TextFile{path:{:?},body:{:?}}",&self.path,&self.body)
//     }
// }


pub fn read(path: &str) -> Result<TextFile,String> {
    let f = abspath(path);
    match f {
        Ok(p) => read_to_string(p),
        Err(e) => Err(format!("{:}",e))
    }
}

pub fn abspath(relative_path: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let path = PathBuf::from(relative_path);
    return fs::canonicalize(path);
}

fn read_to_string(p: PathBuf) -> Result<TextFile,String> {
    if p.is_file() {
        let path = p.to_str().unwrap();
        let mut s = String::new();
        let f = File::open(path);
        if f.is_err() {
            return Err(format!("{:}",&f.unwrap_err()))
        }
        &f.unwrap().read_to_string(&mut s);
        return Ok(TextFile::new(path,&s));
    } else {
        let m = "No such file";
        return Err(m.to_string());
    }
}
