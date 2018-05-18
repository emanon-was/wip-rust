use pound::cfg::Directives;
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

impl Decode for Directives<BackEnd> {
    fn decode(&self) -> String {
        let Directives(v) = self;
        return format!("BackEnd\n{}\nEnd", v.decode().indent());
    }
}

impl Decode for BackEnd {
    fn decode(&self) -> String {
        match &self {
            BackEnd::Address(s) => format!("Address\t{}", s),
            BackEnd::Port(i) => format!("IgnoreCase\t{}", i),
            BackEnd::Disabled(b) => format!("Disabled\t{}", (*b as i32)),
            BackEnd::Cipher(s) => format!("HeadRequire\t{}", s),
            BackEnd::Priority(i) => format!("HeadDeny\t{}", i),
            BackEnd::TimeOut(i) => format!("DnyScale\t{}", i),
            BackEnd::ConnTO(i) => format!("Redirect\t{}", i),
        }
    }
}
