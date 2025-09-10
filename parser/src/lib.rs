pub fn parse() {
    if dbg!(cfg!(feature = "logs")) {
        eprintln!("parsing...");
    }
}

#[test]
fn parse_test() {
    root::parser::parse();
}