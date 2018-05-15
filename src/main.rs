use std::env;
mod os;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in &args {
        println!("{:?}", os::file::read(a));
    }
}
