use std::num::ParseIntError;
use std::str::FromStr;
use super::base_types::*;
use std::time::Duration;
use asai_macro::{ FromLine };
use crate::structure::{FromLine, InvalidValue};

impl<'a> TryFrom<LineField<'a>> for Duration {
    type Error = InvalidValue;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        let v: Vec<u32> = value.value()
            .split(":")
            .flat_map(|x| x.split('.'))
            .map(str::parse)
            .collect::<Result<Vec<u32>, ParseIntError>>()?;
        let hours = v[0];
        let minutes = hours * 60 + v[1];
        let seconds = minutes * 60 + v[2];
        let milliseconds = seconds * 1000 + v[3] * 10;
        Ok(Duration::from_millis(milliseconds as u64))
    }
}

#[derive(FromLine, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Event<'a> {
    #[name("Layer")]
    layer: u32,
    #[name("Start")]
    start: Duration,
    #[name("End")]
    end: Duration,
    #[name("Style")]
    #[default("Default")]
    style: &'a str,
    #[name("Actor")]
    #[default("")]
    actor: &'a str,
    #[name("MarginL")]
    #[default(0)]
    margin_l: u32,
    #[name("MarginR")]
    #[default(0)]
    margin_r: u32,
    #[name("MarginV")]
    #[default(0)]
    margin_v: u32,
    #[name("Effect")]
    #[default("")]
    effect: &'a str,
    #[name("Text")]
    text: &'a str,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventKey {
    Comment,
    Dialogue,
}

impl FromStr for EventKey {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Comment" => Ok(Self::Comment),
            "Dialogue" => Ok(Self::Dialogue),
            _ => Err(InvalidValue)
        }
    }
}

// Workaround to make macros work in this context
mod asai {
    pub use crate::*;
}