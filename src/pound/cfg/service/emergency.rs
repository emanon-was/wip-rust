use pound::cfg::Directives;
use pound::fmt::Decode;
use pound::fmt::Indent;

#[allow(dead_code)]
pub enum Emergency {
    Address(String),
    Port(i32),
}

impl Decode for Emergency {
    fn decode(&self) -> String {
        match &self {
            Emergency::Address(s) => format!("Address\t{}", s),
            Emergency::Port(i) => format!("IgnoreCase\t{}", i),
        }
    }
}

impl Decode for Directives<Emergency> {
    fn decode(&self) -> String {
        let Directives(v) = self;
        return format!("Emergency\n{}\nEnd", v.decode().indent());
    }
}
