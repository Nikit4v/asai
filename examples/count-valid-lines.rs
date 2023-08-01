use asai::iter::parse_str;
use std::env::args;

// static DATA: &'static str = include_str!("../test_data/test.ass");

fn main() {
    let mut args = args();
    let data = std::fs::read_to_string(args.nth(1).expect("No path provided.")).unwrap();
    let mut lines_count: u64 = 0;
    for el in parse_str(&data) {
        if let Ok(el) = el {
            if el.is_line() {
                lines_count += 1
            }
        }
    }
    println!("{lines_count}")
}
