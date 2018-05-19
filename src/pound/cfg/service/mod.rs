use pound::cfg::Block;
use pound::fmt::Decode;
use pound::fmt::Indent;

pub mod backend;
pub mod emergency;
pub mod session;

#[allow(dead_code)]
pub enum Service {
    URL(String),
    IgnoreCase(bool),
    HeadRequire(String),
    HeadDeny(String),
    DnyScale(bool),
    Disabled(bool),
    Redirect(String),
    RedirectWithCode(i32, String),
    BackEnd(Block<backend::BackEnd>),
    //Emergency(Block<emergency::Emergency>),
    Session(Block<session::Session>),
}

impl Decode for Block<Service> {
    fn decode(&self) -> String {
        let Block(v) = &self;
        return format!("Service\n{}\nEnd", v.decode().indent());
    }
}

impl Decode for Service {
    fn decode(&self) -> String {
        match &self {
            Service::URL(s) => format!("URL\t{}", s.decode()),
            Service::IgnoreCase(b) => format!("IgnoreCase\t{}", b.decode()),
            Service::HeadRequire(s) => format!("HeadRequire\t{}", s.decode()),
            Service::HeadDeny(s) => format!("HeadDeny\t{}", s.decode()),
            Service::DnyScale(b) => format!("DnyScale\t{}", b.decode()),
            Service::Disabled(b) => format!("Disabled\t{}", b.decode()),
            Service::Redirect(s) => format!("Redirect\t{}", s.decode()),
            Service::RedirectWithCode(i, s) => format!("Redirect\t{}\t{}", i.decode(), s.decode()),
            Service::BackEnd(x) => x.decode(),
            //Service::Emergency(x) => x.decode(),
            Service::Session(x) => x.decode(),
        }
    }
}
