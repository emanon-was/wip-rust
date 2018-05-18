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
