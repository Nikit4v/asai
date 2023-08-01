use std::convert::Infallible;
use super::InvalidValue;

macro_rules! gen_num {
    ($T: ty) => {
        impl<'a> TryFrom<LineField<'a>> for $T {
            type Error = InvalidValue;

            fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
                Ok(value.value().parse::<$T>()?.into())
            }
        }
    };
}

gen_num!(u8);
gen_num!(u16);
gen_num!(u32);
gen_num!(u64);
gen_num!(i8);
gen_num!(i16);
gen_num!(i32);
gen_num!(i64);
gen_num!(f32);
gen_num!(f64);

/// Color in ass file
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color { a: u8, b: u8, g: u8, r: u8 }
impl<'a> TryFrom<LineField<'a>> for Color {
    type Error = InvalidValue;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        let value = value.value().strip_prefix("&H").ok_or(InvalidValue)?;
        let value = value.strip_suffix("&").ok_or(InvalidValue)?;
        match value.len() {
            6 => {
                let b: &str = &value[0..2];
                let g: &str = &value[2..4];
                let r: &str = &value[4..6];
                Ok(Color {
                    a: 255,
                    b: b.parse()?,
                    g: g.parse()?,
                    r: r.parse()?,
                })
            }
            8 => {
                let a: &str = &value[0..2];
                let b: &str = &value[2..4];
                let g: &str = &value[4..6];
                let r: &str = &value[6..8];
                Ok(Color {
                    a: a.parse()?,
                    b: b.parse()?,
                    g: g.parse()?,
                    r: r.parse()?,
                })
            }
            _ => Err(InvalidValue)
        }
    }
}

impl<'a> TryFrom<LineField<'a>> for bool {
    type Error = InvalidValue;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        match value.into() {
            "0" => Ok(false),
            "-1" => Ok(true),
            _ => Err(InvalidValue)
        }
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
/// Basic Line in ass. Notably, last field can contain commas, while other not.
pub struct LineField<'a>(&'a str);

impl<'a> LineField<'a> {
    pub fn new(s: &'a str) -> Self {
        LineField(s)
    }
}

impl<'a> LineField<'a> {
    pub fn value(&self) -> &str {
        self.0
    }
}

impl<'a> TryFrom<LineField<'a>> for &'a str {
    type Error = Infallible;

    fn try_from(value: LineField<'a>) -> Result<Self, Self::Error> {
        Ok(value.0)
    }
}