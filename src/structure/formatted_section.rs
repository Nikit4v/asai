use std::marker::PhantomData;
use crate::structure::{FromLine, FromLines};

pub struct FormattedSection<'a, L: FromLine<'a>> where Self: Sized {
    pd: PhantomData<&'a str>,
    lines: Vec<L>
}

// pub trait FormattedSection<L: for <'a> FromLine<'a>> {
//
// }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormattedSectionParseError {
    FormatNotFound
}

impl<'a, L: FromLine<'a>> FromLines<'a> for FormattedSection<'a, L> {
    type Err = FormattedSectionParseError;

    fn from_lines(lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err> {
        let format = lines.iter().find(|x| x.0 == "Format").ok_or(FormattedSectionParseError::FormatNotFound)?.1;
        Ok(Self {
            pd: Default::default(),
            lines: lines
            .iter().map(|(name, value)|
                L::from_line(value, format)
            ).filter_map(|x| x.ok()).collect()
        })
    }
}