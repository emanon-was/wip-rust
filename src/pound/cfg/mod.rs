use pound::fmt::Decode;

pub struct Block<T>(pub Vec<T>)
where
    T: Decode;

// #[derive(Debug)]
// pub struct Services {
//     url: String,
//     ignore_case: Option<bool>,
//     head_require: Option<String>,
//     head_deny: Option<String>,
//     dyn_scale: Option<bool>,
//     disabled: Option<bool>,
//     back_end: Option<Vec<BackEnd>>,
//     redirect: Option<(i32, String)>,
//     emergency: Option<Emergency>,
//     session: Option<Session>,
// }

// #[derive(Debug, Eq, PartialEq)]
// pub struct BackEnd {
//     address: String,
//     port: i32,
//     disabled: bool,
//     cipher: Option<String>,
//     priority: Option<i8>,
//     time_out: Option<i32>,
//     connection_time_out: Option<i32>,
// }

// #[derive(Debug)]
// pub struct Session {
//     pub kind: SessionKind,
//     pub id: String,
//     pub ttl: i32,
// }

// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[allow(dead_code)]
// pub enum SessionKind {
//     IP,
//     Basic,
//     URL,
//     Params,
//     Cookie,
//     Header,
// }

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
pub mod service;
