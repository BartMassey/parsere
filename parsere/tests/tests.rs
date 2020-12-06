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
            match e.downcast_ref::<parsere::Error>() {
                Some(parsere::Error::Match { ref re, ref txt }) => {
                    assert_eq!(re, &RE);
                    assert_eq!(txt, &TXT);
                }
                Some(e) => panic!("wrong error: {:?}", e),
                None => panic!("downcast failed: {:?}", e),
            }
        }
        Ok(eg) => panic!("no error: {:?}", eg),
    }
}
