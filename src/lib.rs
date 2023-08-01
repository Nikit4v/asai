use crate::structure::FromLine;

pub mod iter;
pub mod structure;

pub fn parse_str(s: &str) -> structure::Ass {
    structure::Ass::from_elements(iter::parse_str(s))
}