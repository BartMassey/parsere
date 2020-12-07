pub use parsere_derive::ParseRe;

#[derive(Debug, Clone)]
pub enum Error {
    Mismatch { re: &'static str, txt: String },
    NoGroup { re: &'static str, txt: String, index: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Mismatch { re, txt } =>
                write!(f, "regex {} does not match text {}",
                       re, txt),
            Error::NoGroup { re, txt, index } =>
                write!(f, "no capture group {} for regex {} on text {}",
                       re, txt, index),
        }
    }
}

impl std::error::Error for Error {
}
