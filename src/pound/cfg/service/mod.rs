use pound::cfg::Directives;
use pound::fmt::Decode;
use pound::fmt::Indent;

pub mod backend;
pub mod emergency;
// mod session;

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
    BackEnd(Directives<backend::BackEnd>),
    // Emergency(emergency::Directives),
    // Session(session::Directives),
}

impl Decode for Directives<Service> {
    fn decode(&self) -> String {
        let Directives(v) = &self;
        return format!("Service\n{}\nEnd", v.decode().indent());
    }
}

impl Decode for Service {
    fn decode(&self) -> String {
        match &self {
            Service::URL(s) => format!("URL\t{}", s),
            Service::IgnoreCase(b) => format!("IgnoreCase\t{}", (*b as i32)),
            Service::HeadRequire(s) => format!("HeadRequire\t{}", s),
            Service::HeadDeny(s) => format!("HeadDeny\t{}", s),
            Service::DnyScale(b) => format!("DnyScale\t{}", (*b as i32)),
            Service::Disabled(b) => format!("Disabled\t{}", (*b as i32)),
            Service::Redirect(s) => format!("Redirect\t{}", s),
            Service::RedirectWithCode(i, s) => format!("Redirect\t{}\t{}", i, s),
            Service::BackEnd(x) => x.decode(),
            // Service::Emergency(x) => x.decode().indent(),
            // Service::Session(x) => x.decode().indent(),
        }
    }
}
