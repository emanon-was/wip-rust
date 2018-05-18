use pound::cfg::Block;
use pound::fmt::Decode;
use pound::fmt::Indent;

#[allow(dead_code)]
pub enum BackEnd {
    Address(String),
    Port(i32),
    Disabled(bool),
    Cipher(String),
    Priority(i8),
    TimeOut(i32),
    ConnTO(i32),
}

impl Decode for Block<BackEnd> {
    fn decode(&self) -> String {
        let Block(v) = self;
        return format!("BackEnd\n{}\nEnd", v.decode().indent());
    }
}

impl Decode for BackEnd {
    fn decode(&self) -> String {
        match &self {
            BackEnd::Address(s) => format!("Address\t{}", s.decode()),
            BackEnd::Port(i) => format!("IgnoreCase\t{}", i.decode()),
            BackEnd::Disabled(b) => format!("Disabled\t{}", b.decode()),
            BackEnd::Cipher(s) => format!("HeadRequire\t{}", s.decode()),
            BackEnd::Priority(i) => format!("HeadDeny\t{}", i.decode()),
            BackEnd::TimeOut(i) => format!("DnyScale\t{}", i.decode()),
            BackEnd::ConnTO(i) => format!("Redirect\t{}", i.decode()),
        }
    }
}
