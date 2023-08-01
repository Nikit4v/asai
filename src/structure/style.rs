use asai_macro::FromLine;
use crate::structure::InvalidValue;
use super::base_types::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BorderStyle {
    Outline,
    Opaque,
}

impl<'a> TryFrom<LineField<'a>> for BorderStyle {
    type Error = InvalidValue;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        let i: u8 = value.value().parse()?;
        match i {
            1 => Ok(Self::Outline),
            3 => Ok(Self::Opaque),
            _ => Err(InvalidValue)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl<'a> TryFrom<LineField<'a>> for Alignment {
    type Error = InvalidValue;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        let i: u8 = value.value().parse()?;
        match i {
            1 => {Ok(Self::BottomLeft)}
            2 => {Ok(Self::BottomCenter)}
            3 => {Ok(Self::BottomRight)}
            4 => {Ok(Self::CenterLeft)}
            5 => {Ok(Self::CenterCenter)}
            6 => {Ok(Self::CenterRight)}
            7 => {Ok(Self::TopLeft)}
            8 => {Ok(Self::TopCenter)}
            9 => {Ok(Self::TopRight)}
            _ => Err(InvalidValue)
        }
    }
}

#[derive(FromLine, Debug, Clone, PartialEq)]
pub struct Style<'a> {
    #[name("Name")]
    name: LineField<'a>,
    #[name("Fontname")]
    font_name: LineField<'a>,
    #[name("Fontsize")]
    font_size: u32,
    #[name("PrimaryColour")]
    primary_color: Color,
    #[name("SecondaryColour")]
    secondary_color: Color,
    #[name("OutlineColor")]
    outline_color: Color,
    #[name("BackColour")]
    background_color: Color,
    #[name("Bold")]
    bold: bool,
    #[name("Italic")]
    italic: bool,
    #[name("Underline")]
    underline: bool,
    #[name("Strikeout")]
    strikeout: bool,
    #[name("ScaleX")]
    scale_x: f32,
    #[name("ScaleY")]
    scale_y: f32,
    #[name("Spacing")]
    spacing: u32,
    #[name("Angle")]
    angle: f32,
    #[name("BorderStyle")]
    border_style: BorderStyle,
    #[name("Outline")]
    outline: f32,
    #[name("Shadow")]
    shadow: f32,
    #[name("Alignment")]
    alignment: Alignment,
    #[name("MarginL")]
    margin_l: u32,
    #[name("MarginR")]
    margin_r: u32,
    #[name("MarginV")]
    margin_v: u32,
    #[name("Encoding")]
    encoding: u32,
}

// Workaround to make macros work in this context
mod asai {
    pub use crate::*;
}