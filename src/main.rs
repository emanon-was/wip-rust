use std::env;
mod os;
mod pound;
use pound::cfg::service::backend::BackEnd;
use pound::cfg::service::emergency::{Emergency, EmergencyDirectives};
use pound::cfg::service::Service;
use pound::cfg::Block;
use pound::fmt::Decode;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in &args {
        println!("{:?}", os::file::read(a));
        println!(
            "{}",
            Block(vec![
                Service::URL("\"^/test\"".to_string()),
                Service::Disabled(false),
                Service::BackEnd(Block(vec![
                    BackEnd::Address("www.example.co.jp".to_string()),
                    BackEnd::Port(80),
                    BackEnd::Disabled(true),
                ])),
            ]).decode()
        );
    }
}
