use parsere::ParseRe;

const RE: &str = r"^(\d+) ([a-z]+)$";

#[derive(ParseRe, Debug, PartialEq)]
#[re(r"^(\d+) ([a-z]+)$")]
struct Eg {
    x: u8,
    s: String,
}

#[test]
fn works() {
    let eg = Eg::parse_re("50 xyzzy").unwrap();
    assert_eq!(eg, Eg { x: 50, s: "xyzzy".to_string() });
}

macro_rules! error_check {
    ($struc:ident, $txt:expr, $err:pat => $body:tt) => {
        match $struc::parse_re($txt) {
            Err(e) => {
                match e.downcast_ref() {
                    Some($err) => $body,
                    Some(e) => panic!("wrong error: {:?}", e),
                    None => panic!("downcast fail: {:?}", e),
                }
            }
            Ok(eg) => panic!("no error: {:?}", eg),
        }
    };
}

#[test]
fn match_err() {
    const TXT: &str = "50 xyzzy ";
    error_check! { Eg, TXT,
        parsere::Error::Mismatch { re, txt } => {
            assert_eq!(re, &RE);
            assert_eq!(txt, &TXT);
        }
    }
}

const BADRE: &str = r"^(\d+)$";

#[derive(ParseRe, Debug, PartialEq)]
#[re(r"^(\d+)$")]
struct BadReEg {
    x: u8,
    s: String,
}

#[test]
fn group_err() {
    const TXT: &str = "50";
    error_check! { BadReEg, TXT,
        parsere::Error::NoGroup { re, txt, index } => {
            assert_eq!(re, &BADRE);
            assert_eq!(txt, &TXT);
            assert_eq!(index, &2);
        }
    }
}

#[derive(ParseRe, Debug, PartialEq)]
#[re(r"^(\d+$")]
struct NotReEg {
    x: u8,
}

#[test]
fn not_re_err() {
    const TXT: &str = "50";
    error_check! { NotReEg, TXT,
        regex::Error::Syntax(_) => ()
    }
}
