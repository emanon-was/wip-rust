pub trait Indent {
    fn indent(&self) -> String;
}

impl Indent for str {
    fn indent(&self) -> String {
        return self.to_string().indent();
    }
}

impl Indent for String {
    fn indent(&self) -> String {
        let l: Vec<String> = self.split('\n').map(|x| "\t".to_owned() + x).collect();
        return l.join("\n");
    }
}

pub trait Decode {
    fn decode(&self) -> String;
}

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode(&self) -> String {
        let s: Vec<String> = self.iter().map(|t| t.decode()).collect();
        return s.join("\n");
    }
}

impl Decode for String {
    fn decode(&self) -> String {
        self.to_owned()
    }
}

impl Decode for i8 {
    fn decode(&self) -> String {
        self.to_string()
    }
}

impl Decode for i32 {
    fn decode(&self) -> String {
        self.to_string()
    }
}

impl Decode for bool {
    fn decode(&self) -> String {
        (*self as i32).to_string()
    }
}
