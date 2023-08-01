#![allow(dead_code)]

pub mod base_types;
pub mod event;
pub mod style;
pub mod formatted_section;

use std::convert::Infallible;
use crate::iter::{Element, Elements};
use crate::structure::event::Event;
use std::error::Error;
use std::marker::PhantomData;
use std::str::FromStr;

pub struct Ass<'a> {
    pub info: ScriptInfo<'a>,
    pub styles: Styles<'a>,
    pub events: Events<'a>,
}



pub trait FromLines<'a> where Self: Sized {
    type Err;
    fn from_lines(lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SectionParseError {
    SectionNotFound,
    CannotParseSection,
}

impl<'a> Ass<'a> {
    pub fn from_elements<T: Iterator<Item = &'a str>>(iter: Elements<'a, T>) -> Self {
        let mut info: ScriptInfo = Default::default();
        let mut styles: Styles = Default::default();
        let mut events: Option<Events> = Default::default();
        let mut current_section: Option<&'a str> = Default::default();
        let mut lines: Vec<(&'a str, &'a str)> = vec![];
        for i in iter {
            if i.is_err() {
                continue;
            }
            match i.unwrap() {
                Element::SectionDefinition(name) => {
                    if let Some(name) = current_section {
                        match name {
                            "Script Info" => info = ScriptInfo::from_lines(&lines).unwrap(),
                            "V4+ Styles" => styles = Styles::from_lines(&lines).unwrap(),
                            "Events" => events = Some(Events::from_lines(&lines).unwrap()),
                            _ => (),
                        }
                    }
                    current_section = Some(name);
                    lines.clear();
                }
                Element::Line { name, value } if name != "!" => lines.push((name, value)),
                _ => (), // Ignore comments
            }
        }

		if let Some(current_section) = current_section {
			match current_section {
				"Script Info" => info = ScriptInfo::from_lines(&lines).unwrap(),
				"V4+ Styles" => styles = Styles::from_lines(&lines).unwrap(),
				"Events" => events = Some(Events::from_lines(&lines).unwrap()),
				_ => (),
			}
		}

        Self {
            info,
            styles,
            events: events.unwrap(),
        }
    }

    pub fn parse_section<'b, T: FromLines<'b>>(name: &str, s: &'b str) -> Result<T, SectionParseError> {
        let mut current_section: Option<&'b str> = Default::default();
        let mut lines: Vec<(&'b str, &'b str)> = vec![];
        let iter = crate::iter::parse_str(s);
        for i in iter {
            if i.is_err() {
                continue;
            }
            match i.unwrap() {
                Element::SectionDefinition(section_name) => {
                    if let Some(name_) = current_section {
                        if name_ == name {
                            return T::from_lines(&lines).map_err(|_| SectionParseError::CannotParseSection)
                        }
                    }
                    current_section = Some(section_name);
                    lines.clear();
                }
                Element::Line { name, value } if name != "!" => lines.push((name, value)),
                _ => (), // Ignore comments
            }
        }
        if let Some(name_) = current_section {
            if name_ == name {
                return T::from_lines(&lines).map_err(|_| SectionParseError::CannotParseSection)
            }
        }

        Err(SectionParseError::SectionNotFound)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Authors<'a> {
    script: Option<&'a str>,
    translation: Option<&'a str>,
    editing: Option<&'a str>,
    timing: Option<&'a str>,
    updated_by: Option<&'a str>,
    update_details: Option<&'a str>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum ScriptCollisionsType {
    #[default]
    Normal,
    Reverse,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ColorDepth(u8);

impl<N: Into<u8>> From<N> for ColorDepth {
    fn from(value: N) -> Self {
        ColorDepth(value.into())
    }
}

impl Default for ColorDepth {
    fn default() -> Self {
        8.into()
    }
}

impl FromStr for ColorDepth {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u8>()?.into())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TimeScale(f64);

impl<N: Into<f64>> From<N> for TimeScale {
    fn from(value: N) -> Self {
        TimeScale(value.into())
    }
}

impl Default for TimeScale {
    fn default() -> Self {
        100.into()
    }
}

impl FromStr for TimeScale {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<f64>()?.into())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Resolution {
    x: Option<u64>,
    y: Option<u64>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum WrapStyle {
    #[default]
    SmartConstant = 0,
    EndOfLine = 1,
    NoWrap = 2,
    SmartVariable = 3,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ScriptInfo<'a> {
    pub resolution: Resolution,
    pub authors: Authors<'a>,
    pub title: Option<&'a str>,
    pub sync_point: Option<&'a str>,
    pub version: &'a str,
    pub timescale: f64,
    pub color_depth: ColorDepth,
    pub collisions: ScriptCollisionsType,
    pub wrap_style: WrapStyle,
}

#[derive(Debug, Clone, Default)]
pub struct Styles<'a> {
    pd: PhantomData<&'a str>,
}

#[derive(Debug, Clone, Default)]
pub struct Events<'a> {
    pub events: Vec<Event<'a>>,
}

#[derive(Debug)]
pub struct InvalidValue;

impl<E: Error> From<E> for InvalidValue {
    fn from(_value: E) -> Self {
        InvalidValue
    }
}

impl FromStr for ScriptCollisionsType {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(Self::Normal),
            "Reverse" => Ok(Self::Reverse),
            _ => Err(InvalidValue),
        }
    }
}

impl FromStr for WrapStyle {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let desc: u8 = s.parse()?;
        match desc {
            0 => Ok(Self::SmartConstant),
            1 => Ok(Self::EndOfLine),
            2 => Ok(Self::NoWrap),
            3 => Ok(Self::SmartVariable),
            _ => Err(InvalidValue),
        }
    }
}

impl<'a> FromLines<'a> for ScriptInfo<'a> {
    type Err = Infallible;

    fn from_lines(lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err> {
        let mut info: Self = Default::default();
        for i in lines {
            match i.0 {
                "Title" => info.title = Some(i.1),
                i if i.starts_with("Original") || i.contains("Update") => {
                    println!("INFO: Authors info parsing is not implemented yet. Field {:?} will be ignored.", i)
                } // Ignore it for now.
                "Sync Point" => info.sync_point = Some(i.1), // Never seen this field, idk what format of this field is.
                "Script Type" => info.version = i.1,
                "Timer" => info.timescale = i.1.parse().unwrap_or_default(),
                "Collisions" => info.collisions = i.1.parse().unwrap_or_default(),
                "PlayDepth" => info.color_depth = i.1.parse().unwrap_or_default(),
                "PlayResY" => info.resolution.y = i.1.parse().ok(),
                "PlayResX" => info.resolution.x = i.1.parse().ok(),
                "WrapStyle" => info.wrap_style = i.1.parse().unwrap_or_default(),
                _ => {} // For unknown fields
            }
        }

        // Set ASS version if no version specified.
        if info.version.is_empty() {
            info.version = "V4.00+"
        }

        Ok(info)
    }
}

impl<'a> FromLines<'a> for Styles<'a> {
    type Err = ();

    fn from_lines(_lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl<'a> FromLines<'a> for Events<'a> {
    type Err = Infallible;

    fn from_lines(lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err> {
		// println!("Here");
        let format = lines
            .iter()
            .find(|x| x.0 == "Format")
            .map(|x| x.1)
            .or_else(|| {
                Some("Layer, Start, End, Style, Actor, MarginL, MarginR, MarginV, Effect, Text")
            })
            .unwrap();
		// println!("{}", format);
		// let events = vec![];
		// for (name, value) in lines {
		// 	if name == &"Format" {
		// 		continue;
		// 	}
		// 	let ev = Event::from_line(value, format);
		// }
        let events: Vec<Event> = lines
            .iter()
            .filter(|(name, value)| *name != "Format")
            .map(|x| FromLine::from_line(x.1, format))
            .filter_map(|x| x.ok())
            .collect();
        Ok(Self { events })
    }
}

pub trait FromLine<'a>
where
    Self: Sized,
{
    fn from_line(item: &'a str, format: &str) -> Result<Self, InvalidValue>;
}
