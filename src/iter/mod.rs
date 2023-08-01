use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
pub enum Element<'a> {
    SectionDefinition(&'a str),
    Line { name: &'a str, value: &'a str },
    Comment(&'a str),
}

impl<'a> Element<'a> {
    pub fn is_line(&self) -> bool {
        match self {
            Element::Line {..} => true,
            _ => false
        }
    }

    pub fn is_section(&self) -> bool {
        match self {
            Element::SectionDefinition(_) => true,
            _ => false
        }
    }

    pub fn is_comment(&self) -> bool {
        match self {
            Element::Comment(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidElement<'a>(pub &'a str);

pub struct Elements<'a, T: Iterator<Item = &'a str>> {
    iter: T,
}

pub fn parse_str(s: &str) -> Elements<Lines> {
    Elements::new(s.lines())
}

impl<'a, T: Iterator<Item = &'a str>> Elements<'a, T> {
    pub fn new(iter: T) -> Self {
        Self { iter }
    }
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Elements<'a, T> {
    type Item = Result<Element<'a>, InvalidElement<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = self.iter.next();
        while v == Some("") {
            v = self.iter.next();
        }
        match v {
            Some(line) if line.starts_with(';') => Some(Ok(Element::Comment(
                line.strip_prefix(";").unwrap()
            ))),
            Some(line) if line.starts_with('[') && line.ends_with(']') => {
                Some(Ok(Element::SectionDefinition(
                    line.strip_prefix('[')
                        .and_then(|x| x.strip_suffix(']'))
                        .unwrap(),
                )))
            }
            Some(line)
                if line.contains(": ") =>
            {
                let mut parts = line.splitn(2, ": ");
                Some(Ok(Element::Line {
                    name: parts.next().unwrap(),
                    value: parts.next().unwrap(),
                }))
            }
            Some(line) => Some(Err(InvalidElement(line))),
            None => None,
        }
    }
}
