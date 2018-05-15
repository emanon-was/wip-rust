use std::fmt;

trait Decode {
    pub fn decode(&self) -> Vec<String>
}


#[derive(Debug)]
pub struct Emergency {
    pub address: String,
    pub port: i32,
}

impl fmt::Display for Emergency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Emergency").unwrap();
        writeln!(f, "  Address {}", &self.address).unwrap();
        writeln!(f, "  Port    {}", &self.port).unwrap();
        writeln!(f, "End").unwrap();
        return writeln!(f,"");
    }
}

#[derive(Debug)]
pub struct Session {
    pub kind: SessionKind,
    pub id: String,
    pub ttl: i32,
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Session").unwrap();
        writeln!(f, "  Type {}",  &self.kind.as_str()).unwrap();
        writeln!(f, "  ID   {:}", &self.id).unwrap();
        writeln!(f, "  TTL  {:}", &self.ttl).unwrap();
        writeln!(f, "End").unwrap();
        return writeln!(f, "");
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum SessionKind {
    IP,
    Basic,
    URL,
    Parm,
    Cookie,
    Header,
}

impl SessionKind {
    fn as_str(&self) -> String {
        match self {
            SessionKind::IP => "IP",
            SessionKind::Basic => "BASIC",
            SessionKind::URL => "URL",
            SessionKind::Parm => "PARM",
            SessionKind::Cookie => "COOKIE",
            SessionKind::Header => "HEADER",
        }.to_string()
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct BackEnd {
    address: String,
    port: i32,
    disabled: bool,
    cipher: Option<String>,
    priority: Option<i8>,
    time_out: Option<i32>,
    connection_time_out: Option<i32>,
}

// #[derive(Debug)]
// pub struct Services {
//     url: String,
//     ignore_case: Option<bool>,
//     head_require: Option<String>,
//     head_deny: Option<String>,
//     dyn_scale: Option<bool>,
//     disabled: Option<bool>,
//     back_end: Option<Vec<BackEnd>>,
//     redirect: Option<(i32,String)>,
//     emergency: Option<Emergency>,
//     session: Option<Session>,
// }

