use std::error::Error;

#[derive(Debug, Clone)]
pub enum MyError {
    Mismatch { re: &'static str, txt: String },
    NoGroup { re: &'static str, txt: String, index: usize },
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Mismatch { re, txt } =>
                write!(f, "regex {} does not match text {}",
                       re, txt),
            MyError::NoGroup { re, txt, index } =>
                write!(f, "no capture group {} for regex {} on text {}",
                       re, txt, index),
        }
    }
}

impl Error for MyError {}


#[derive(Debug)]
struct Eg {
    x: u8,
}

const RE: &'static str = r"^([1-9][0-9]*)$";

impl Eg {
    fn parse_re(txt: &str) ->
        Result<Self, Box<dyn std::error::Error>>
    {
        use regex::Regex;
        use once_cell::unsync::Lazy;
        let re = Lazy::new(|| Regex::new(RE));
        let re: &Result<Regex, regex::Error> = Lazy::force(&re);
        let captures = match re {
            Ok(re) => {
                let captures: Option<regex::Captures> = re.captures(txt);
                captures.ok_or_else(|| {
                    Box::new(MyError::Mismatch {
                        re: RE,
                        txt: txt.to_string(),
                    })
                })?
            }
            Err(e) => return Err(Box::new(e.clone())),
        };
        Ok(Self {
            x: captures
                .get(1)
                .ok_or_else(|| {
                    Box::new(MyError::NoGroup { re: RE, txt: txt.to_string(), index: 1 })
                })?
                .as_str()
                .parse()?,
        })
    }
}

/*
// Passes as expected.
fn main() {
    let e: Box<dyn Error> =
        Box::new(MyError::Mismatch{re: "pfft", txt: "huh".to_string()});
    assert!(e.downcast_ref::<MyError>().is_some());
}
*/

fn main() {
    const TXT: &str = "50 ";
    let result: Result<Eg, Box<dyn Error>> = Eg::parse_re(TXT);
    match result {
        Err(e) => {
            match e.downcast_ref::<MyError>() {
                Some(&MyError::Mismatch { re, ref txt }) => {
                    assert_eq!(re, RE);
                    assert_eq!(txt, TXT);
                }
                Some(e) => panic!("wrong error: {:?}", e),
                None => panic!("downcast fail: {:?}", e),
            }
        }
        Ok(eg) => panic!("no error: {:?}", eg),
    }
}
