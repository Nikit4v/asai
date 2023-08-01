use std::env::args;
use asai::parse_str;
use asai::structure::{ Events, Styles };
use asai::structure::event::Event;
use asai::structure::style::Style;
use asai_macro::FromLine;

pub fn main() {
    let mut args = args();
    let t = std::time::Instant::now();
    let data = std::fs::read_to_string(args.nth(1).expect("No path provided")).unwrap();
    let events: Events<Event> = parse_str(&data).events;
    let styles: Styles = parse_str(&data).styles;
}