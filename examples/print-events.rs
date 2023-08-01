use std::env::args;
use asai::parse_str;
use asai::structure::Events;
use asai::structure::event::Event;
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
    let my_events: Events<MyEvent> = parse_str(&data).events; // Using custom structure
    println!("Custom: {:?}", t.elapsed());
    let t = std::time::Instant::now();
    let builtin_events: Events<Event> = parse_str(&data).events; // Using builtin one
    println!("Builtin: {:?}", t.elapsed());
    println!("{:#?}", my_events.events[my_events.events.len() - 1]);
    println!("{:#?}", builtin_events.events[builtin_events.events.len() - 1]);
}