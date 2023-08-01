# asai

A rust library for parsing .ass files, which are used for storing subtitles in the Advanced SubStation Alpha format.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
asai = "0.1.0"
```

## Usage 

If you want to simply parse ass, you can use parse_str from asai's root:
```rust
use asai::structure::Ass;

fn main() {
    let data = "...";
    let ass = asai::parse_str(data);
    println!("{:?}", ass.styles); // Get styles
    println!("{:?}", ass.info); // Get info
    println!("{:?}", ass.events); // Get events
}
```

Another approach is to use custom structure to represent ass. It allows us to set defaults and parse only required fields.
There is an example:
```rust
use asai_macro::FromLine;
use asai::structure::event::EventKey;
use asai::structure::Ass;

#[derive(FromLine, Debug)]
struct MyEvent<'a> {
    #[name("Text")]
    text: &'a str,
    #[name("Custom")]
    #[default("Some default value")]
    custom_field: &'a str
}

fn main() {
    let data = "...";
    let my_events: FormattedSection<EventKey, MyEvent> = Ass::parse_section("Events", data).unwrap();
}
```