use pound::cfg::Block;
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
            Emergency::Address(s) => format!("Address\t{}", s.decode()),
            Emergency::Port(i) => format!("IgnoreCase\t{}", i.decode()),
        }
    }
}

impl Decode for Block<Emergency> {
    fn decode(&self) -> String {
        let Block(v) = self;
        return format!("Emergency\n{}\nEnd", v.decode().indent());
    }
}
