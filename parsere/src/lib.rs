pub use parsere_derive::ParseRe;

#[derive(Debug, Clone)]
pub enum Error {
    Match { re: &'static str, txt: String },
    Capture { re: &'static str, txt: String, index: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Match { re, txt } =>
                write!(f, "regex {} does not match text {}",
                       re, txt),
            Error::Capture { re, txt, index } =>
                write!(f, "no capture group {} for regex {} on text {}",
                       re, txt, index),
        }
    }
}

impl std::error::Error for Error {
}
