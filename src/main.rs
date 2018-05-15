use std::env;
mod os;
mod pound;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in &args {
        println!("{:?}", os::file::read(a));
        println!("{:}",pound::Emergency {address: "test".to_string(), port: 100});
        println!("{:}", pound::Session {
            kind: pound::SessionKind::IP,
            id: "8.8.8.8".to_string(),
            ttl: 5000,
        });
    }
}
