use parsere::ParseRe;

#[derive(ParseRe, Debug)]
#[re(r"(\d+) ([a-z]+)")]
struct Eg {
    x: u8,
    s: String,
}

fn main() {
    let test = "50 xyzzy";
    let eg = Eg::parse_re(test).unwrap();
    println!("{} {}", eg.x, eg.s);
}
