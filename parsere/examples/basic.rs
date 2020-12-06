use parsere::ParseRe;

#[derive(ParseRe, Debug)]
#[re(r"(\d+) ([a-z]+)")]
struct Eg {
    x: u8,
    s: String,
}

fn main() {
    let eg = Eg::parse_re("50 xyzzy").unwrap();
    println!("{:#?}", eg);
}
