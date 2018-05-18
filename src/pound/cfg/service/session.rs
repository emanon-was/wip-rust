use pound::fmt::Decode;
use pound::fmt::Indent;

pub struct Block {
    inner: Vec<Session>,
}

pub enum Session {
    Kind(SessionKind),
    ID(String),
    TTL(i32),
}

pub enum SessionKind {
    IP,
    Basic,
    URL,
    Params,
    Cookie,
    Header,
}

// impl SessionKind {
//     fn as_str(&self) -> &str {
//         match &self {
//             SessionKind::IP => "IP",
//             SessionKind::Basic => "BASIC",
//             SessionKind::URL => "URL",
//             SessionKind::Params => "PARM",
//             SessionKind::Cookie => "COOKIE",
//             SessionKind::Header => "HEADER",
//         }
//     }
// }
