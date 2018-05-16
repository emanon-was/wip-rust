pub trait Indent {
    fn indent(&self, usize) -> String;
}

impl Indent for str {
    fn indent(&self, n: usize) -> String {
        return self.to_string().indent(n);
    }
}

impl Indent for String {
    fn indent(&self, n: usize) -> String {
        let i = "\t".repeat(n);
        let l: Vec<String> = self.split('\n').map(|x| i.to_owned() + x).collect();
        return l.join("\n");
    }
}
