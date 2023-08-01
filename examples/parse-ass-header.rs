use std::env::args;

fn main() {
    let mut args = args();
    let data = std::fs::read_to_string(args.nth(1).expect("No path provided")).unwrap();
    let ass = asai::parse_str(&data);
    println!("{:#?}", ass.info)
}