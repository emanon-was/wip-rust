use pound::fmt::Decode;

pub struct Directives<T>(pub Vec<T>)
where
    T: Decode;

pub mod service;
