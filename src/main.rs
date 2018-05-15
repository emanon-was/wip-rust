use std::env;
mod os;
mod pound_config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in &args {
        println!("{:?}", os::file::read(a));
        println!("{:}",pound_config::Emergency {address: "test".to_string(), port: 100})
    }
}
