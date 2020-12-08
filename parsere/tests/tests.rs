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

#[test]
fn match_err() {
    const TXT: &str = "50 xyzzy ";
    match Eg::parse_re(TXT) {
        Err(e) => {
            match e.downcast_ref() {
                Some(&parsere::Error::Mismatch { re, ref txt }) => {
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
    match BadReEg::parse_re(TXT) {
        Err(e) => {
            match e.downcast_ref() {
                Some(&parsere::Error::NoGroup { re, ref txt, index }) => {
                    assert_eq!(re, BADRE);
                    assert_eq!(txt, TXT);
                    assert_eq!(index, 2);
                }
                Some(e) => panic!("wrong error: {:?}", e),
                None => panic!("downcast fail: {:?}", e),
            }
        }
        Ok(eg) => panic!("no error: {:?}", eg),
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
    match NotReEg::parse_re(TXT) {
        Err(e) => {
            match e.downcast_ref() {
                Some(&regex::Error::Syntax(_)) => {
                    ()
                }
                Some(e) => panic!("wrong error: {:?}", e),
                None => panic!("downcast fail: {:?}", e),
            }
        }
        Ok(eg) => panic!("no error: {:?}", eg),
    }
}
