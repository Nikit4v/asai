use crate::structure::{FromLine, FromLines, InvalidValue};
use std::marker::PhantomData;
use std::ops::Index;
use std::str::FromStr;

pub struct FormattedSection<'a, K: FromStr, L: FromLine<'a>>
where
    Self: Sized,
{
    pd: PhantomData<&'a str>,
    lines: Vec<(K, L)>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FormattedSectionParseError<'a> {
    FormatNotFound,
    InvalidLine(&'a str, &'a str),
}

impl<'a, K: FromStr, L: FromLine<'a>> FromLines<'a> for FormattedSection<'a, K, L> {
    type Err = FormattedSectionParseError<'a>;

    fn from_lines(lines: &[(&'a str, &'a str)]) -> Result<Self, Self::Err> {
        let format = lines
            .iter()
            .find(|x| x.0 == "Format")
            .ok_or(FormattedSectionParseError::FormatNotFound)?
            .1;
        let mut lines_ = Vec::with_capacity(lines.len());
        for &line in lines {
            lines_.push((
                line.0
                    .parse::<K>()
                    .map_err(|_| FormattedSectionParseError::InvalidLine(line.0, line.1))?,
                L::from_line(line.1, format)
                    .map_err(|_| FormattedSectionParseError::InvalidLine(line.0, line.1))?,
            ));
        }
        Ok(Self {
            pd: PhantomData::default(),
            lines: lines_,
        })
    }
}

impl<'a, K: FromStr, L: FromLine<'a>> FormattedSection<'a, K, L> {
    pub fn len(&self) -> usize {
        self.lines.len()
    }
}

impl<'a, K: FromStr, L: FromLine<'a>> Index<usize> for FormattedSection<'a, K, L> {
    type Output = (K, L);

    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}
