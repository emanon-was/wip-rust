use std::env;
mod file;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in &args {
        println!("{:?}",file::read(a));
    }
}
