use pound::fmt::Decode;

pub struct Block<T>(pub Vec<T>)
where
    T: Decode;

pub mod service;
