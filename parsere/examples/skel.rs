#[derive(Debug)]
struct Eg {
    x: u8,
}

impl Eg {
    fn parse_re(txt: &str) ->
        std::result::Result<Self, std::boxed::Box<dyn std::error::Error>>
    {
        use regex::Regex;
        use once_cell::unsync::Lazy;
        let re = Lazy::new(|| Regex::new(RE));
        let captures = match Lazy::force(&re) {
            Ok(re) => re.captures(txt).ok_or_else(|| {
                Box::new(parsere::Error::Match { re: RE, txt: txt.to_string() })
            })?,
            Err(e) => return Err(Box::new(e.clone())),
        };
        Ok(Self {
            x: captures
                .get(1)
                .ok_or_else(|| {
                    Box::new(parsere::Error::Capture { re: RE, txt: txt.to_string(), index: 1 })
                })?
                .as_str()
                .parse()?,
        })
    }
}

fn main() {
    let eg = Eg::parse_re("10").unwrap();
    println!("{:#?}", eg);
}
