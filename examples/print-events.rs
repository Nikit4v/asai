use std::env::args;
use asai::structure::{Ass};
use asai::structure::event::{Event, EventKey};
use asai::structure::formatted_section::FormattedSection;
use asai_macro::FromLine;

#[derive(FromLine, Debug)]
struct MyEvent<'a> {
    #[name("Text")]
    text: &'a str,
    #[name("Custom")]
    #[default("Some default value")]
    custom_field: &'a str
}

pub fn main() {
    let mut args = args();
    let t = std::time::Instant::now();
    let data = std::fs::read_to_string(args.nth(1).expect("No path provided")).unwrap();

    let my_events: FormattedSection<EventKey, MyEvent> = Ass::parse_section("Events", &data).unwrap(); // Using custom structure
    println!("Custom: {:?}", t.elapsed());
    let t = std::time::Instant::now();
    let builtin_events: FormattedSection<EventKey, Event> = Ass::parse_section("Events", &data).unwrap(); // Using custom structure
    println!("Builtin: {:?}", t.elapsed());
    println!("{:#?}", my_events[my_events.len() - 1]);
    println!("{:#?}", builtin_events[builtin_events.len() - 1]);
}