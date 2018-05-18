use pound::cfg::Block;
use pound::fmt::Decode;
use pound::fmt::Indent;

#[allow(dead_code)]
pub enum Session {
    Kind(SessionKind),
    ID(String),
    TTL(i32),
}

#[allow(dead_code)]
pub enum SessionKind {
    IP,
    Basic,
    URL,
    Params,
    Cookie,
    Header,
}

impl Decode for SessionKind {
    fn decode(&self) -> String {
        match &self {
            SessionKind::IP => "IP",
            SessionKind::Basic => "BASIC",
            SessionKind::URL => "URL",
            SessionKind::Params => "PARM",
            SessionKind::Cookie => "COOKIE",
            SessionKind::Header => "HEADER",
        }.to_string()
    }
}

impl Decode for Session {
    fn decode(&self) -> String {
        match &self {
            Session::Kind(t) => format!("Type\t{}", t.decode()),
            Session::ID(s) => format!("ID\t{}", s.decode()),
            Session::TTL(i) => format!("TTL\t{}", i.decode()),
        }
    }
}

impl Decode for Block<Session> {
    fn decode(&self) -> String {
        let Block(v) = self;
        return format!("Session\n{}\nEnd", v.decode().indent());
    }
}
