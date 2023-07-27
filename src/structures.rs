use std::collections::HashMap;
use std::str::FromStr;

pub mod section;

pub struct Ass<'a> {
    header: Header<'a>,
    styles: Vec<Style<'a>>,
    events: Vec<Event<'a>>,
}

pub struct Header<'a> {
    headers: HashMap<&'a str, &'a str>,
}

pub enum StyleFormatField {
    Name,
    Fontname,
    Fontsize,
    PrimaryColour,
    SecondaryColour,
    OutlineColour,
    BackColour,
    Bold,
    Italic,
    Underline,
    Strikeout,
    ScaleX,
    ScaleY,
    Spacing,
    Angle,
    BorderStyle,
    Outline,
    Shadow,
    Alignment,
    MarginL,
    MarginR,
    MarginV,
    Encoding,
}
pub enum EventFormatField {
    Layer,
    Start,
    End,
    Style,
    Actor,
    MarginL,
    MarginR,
    MarginV,
    Effect,
    Text,
}

pub struct StyleFormat {
    fields: Vec<StyleFormatField>,
}

pub struct EventFormat {
    fields: Vec<EventFormatField>,
}

pub struct NotAValidField(String);

impl FromStr for StyleFormat {
    type Err = NotAValidField;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for i in s.split(',') {
            let i = i.trim();
            let field = match i {
                "Name" => StyleFormatField::Name,
                "Fontname" => StyleFormatField::Fontname,
                "Fontsize" => StyleFormatField::Fontsize,
                "PrimaryColour" => StyleFormatField::PrimaryColour,
                "SecondaryColour" => StyleFormatField::SecondaryColour,
                "OutlineColour" => StyleFormatField::OutlineColour,
                "BackColour" => StyleFormatField::BackColour,
                "Bold" => StyleFormatField::Bold,
                "Italic" => StyleFormatField::Italic,
                "Underline" => StyleFormatField::Underline,
                "Strikeout" => StyleFormatField::Strikeout,
                "ScaleX" => StyleFormatField::ScaleX,
                "ScaleY" => StyleFormatField::ScaleY,
                "Spacing" => StyleFormatField::Spacing,
                "Angle" => StyleFormatField::Angle,
                "BorderStyle" => StyleFormatField::BorderStyle,
                "Outline" => StyleFormatField::Outline,
                "Shadow" => StyleFormatField::Shadow,
                "Alignment" => StyleFormatField::Alignment,
                "MarginL" => StyleFormatField::MarginL,
                "MarginR" => StyleFormatField::MarginR,
                "MarginV" => StyleFormatField::MarginV,
                "Encoding" => StyleFormatField::Encoding,
                x => return Err(NotAValidField(x.to_string()))
            };
            
        }
        todo!()
    }
}

pub struct Styles<'a> {
    format: StyleFormat,
    styles: Vec<Style<'a>>,
}

pub struct Events<'a> {
    format: EventFormat,
    events: Vec<Event<'a>>,
}

pub struct Style<'a> {
    name: &'a str,
}

pub struct Event<'a> {
    text: &'a str,
}

pub enum AssParseError {}

impl<'a> FromStr for Ass<'a> {
    type Err = AssParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::section::SectionsExt;

    #[test]
    fn test_sections() {
        let data = std::fs::read_to_string("/Users/aleksejzmeevyh/Downloads/_Erai_raws_Masamune_kun_no_Revenge_R_04_1080pMultiple_SubtitleB5C2E59D.ass").unwrap();
        for section in data.sections() {
            println!("{:#?}", section);
        }
        println!(
            "{:#?}",
            data.sections()
                .find(|x| x.name == "Events")
                .expect("Have no events section.")
                .get("Dialogue")
        )
    }
}
