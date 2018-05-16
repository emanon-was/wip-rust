pub mod fmt;
use self::fmt::Indent;

#[derive(Debug)]
pub struct Services {
    url: String,
    ignore_case: Option<bool>,
    head_require: Option<String>,
    head_deny: Option<String>,
    dyn_scale: Option<bool>,
    disabled: Option<bool>,
    back_end: Option<Vec<BackEnd>>,
    redirect: Option<(i32, String)>,
    emergency: Option<Emergency>,
    session: Option<Session>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackEnd {
    address: String,
    port: i32,
    disabled: bool,
    cipher: Option<String>,
    priority: Option<i8>,
    time_out: Option<i32>,
    connection_time_out: Option<i32>,
}

#[derive(Debug)]
pub struct Emergency {
    pub address: String,
    pub port: i32,
}

#[derive(Debug)]
pub struct Session {
    pub kind: SessionKind,
    pub id: String,
    pub ttl: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum SessionKind {
    IP,
    Basic,
    URL,
    Params,
    Cookie,
    Header,
}

impl SessionKind {
    fn as_str(&self) -> &str {
        match &self {
            SessionKind::IP => "IP",
            SessionKind::Basic => "BASIC",
            SessionKind::URL => "URL",
            SessionKind::Params => "PARM",
            SessionKind::Cookie => "COOKIE",
            SessionKind::Header => "HEADER",
        }
    }
}

pub trait Decode {
    fn entity(&self) -> Vec<String>;
    fn block(&self) -> &str;
    fn decode(&self) -> String {
        let mut a = vec![self.block().to_string()];
        let mut b = self.entity().iter().map(|s| s.indent(1)).collect();
        let mut c = vec!["End".to_string()];
        a.append(&mut b);
        a.append(&mut c);
        return a.join("\n");
    }
}

impl Decode for Session {
    fn block(&self) -> &str {
        "Session"
    }
    fn entity(&self) -> Vec<String> {
        let kind = format!("Type\t{}", &self.kind.as_str());
        let id = format!("ID\t\"{}\"", &self.id);
        let ttl = format!("TTL\t{}", &self.ttl);
        return vec![kind, id, ttl];
    }
}

impl Decode for Emergency {
    fn block(&self) -> &str {
        "Emergency"
    }
    fn entity(&self) -> Vec<String> {
        let address = format!("Address\t{}", &self.address);
        let port = format!("Port\t{}", &self.port);
        return vec![address, port];
    }
}
