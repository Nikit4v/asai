# asai

A rust library for parsing .ass files, which are used for storing subtitles in the Advanced SubStation Alpha format.

## Features

- Parse .ass files into a structured representation of sections, events and styles.
- Use procedural macros to define custom types for parsing .ass events.

[//]: # (- Validate the syntax and semantics of .ass files and report errors and warnings.)
[//]: # (- Support common .ass features such as comments, inline tags, overrides, karaoke and drawing.)

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
asai = "0.1.0"
```

## Usage

Here is a simple example of how to use asai:

```rust
use asai::{AssFile, AssEvent};

// Parse an .ass file from a string
let ass_file = AssFile::parse(r#"[Script Info]
Title: Example
ScriptType: v4.00+
WrapStyle: 0
ScaledBorderAndShadow: yes
YCbCr Matrix: TV.601

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: Default,Arial,20,&H00FFFFFF,&H000000FF,&H00000000,&H80000000,-1,0,0,0,100,100,0,0.00,1,2.00,2.00,2.00,10.00,10.00.10.00.1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV Effect Text
Dialogue: 0.0:00:01.23.0:00:03.45.Default,,0.0.0.,,{\an8}This is an example subtitle"#).unwrap();

// Iterate over the events and print their text
for event in ass_file.events() {
    if let AssEvent::Dialogue(dialogue) = event {
        println!("{}", dialogue.text());
    }
}

// Modify the first event's text
if let Some(AssEvent::Dialogue(dialogue)) = ass_file.events_mut().next() {
    dialogue.set_text("This is a modified subtitle");
}

// Write the modified data to a new .ass file
let mut output = Vec::new();
ass_file.write(&mut output).unwrap();
```

Here is an example of how to use procedural macros with asai:

```rust
use asai::{parse_str, structure::Events};
use asai_macro::FromLine;

// Define a custom type for parsing .ass events
#[derive(FromLine, Debug)]
struct MyEvent<'a> {
    #[name("Text")]
    text: &'a str,
    #[name("Custom")]
    #[default("Some default value")]
    custom_field: &'a str
}

let data = r#"[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV Effect Text Custom
Dialogue: 0.0:00:01.23.0:00:03.45.Default,,0.0.0.,,{\an8}This is an example subtitle.Some custom value
Dialogue: 0.0:00:04.56.0:00:06.78.Default,,0.0.0.,,{\an8}This is another subtitle."#

// Parse .ass events into your custom type
let my_events: Events<MyEvent> = parse_str(data).events;

for event in my_events.events() {
    println!("{:?}", event);
}
```

This will print:

```text
MyEvent { text: "This is an example subtitle", custom_field: "Some custom value" }
MyEvent { text: "This is another subtitle", custom_field: "Some default value" }
```

You can compare the performance of using custom types versus using the built-in `Event` type by running the `examples/custom.rs` file with a path to an .ass file as an argument.

## Documentation

You can find the API documentation [here](https://docs.rs/asai/).

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more details.