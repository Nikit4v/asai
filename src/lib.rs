use crate::structure::FromLine;

pub mod iter;
pub mod structure;

pub fn parse_str<'a, E: FromLine<'a>>(s: &'a str) -> structure::Ass<'a, E> {
    structure::Ass::from_elements(iter::parse_str(s))
}