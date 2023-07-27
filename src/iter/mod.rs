#![deny(clippy::all)]
use std::iter::Peekable;
mod tests;

pub trait ObjectSelector<'a> {
    type Item;
    type ItemIterator;
    fn get(&'a self, name: &str) -> Option<Self::Item>;
    fn get_many(&'a self, name: &'a str) -> Self::ItemIterator;
    fn add(&'a self, item: Self::Item) -> Self;
    fn add_many(&'a self, items: impl Iterator<Item = Self::Item>) -> Self;
}

pub trait SectionsExt<'a> {
    fn sections(&'a self) -> Sections<'a>;
}

impl<'a> SectionsExt<'a> for str {
    fn sections(&self) -> Sections {
        Sections::new(self)
    }
}


pub struct Sections<'a> {
    sections: Vec<Section<'a>>,
}

impl<'a> ObjectSelector<'a> for Sections<'a> {
    type Item = Section<'a>;
    type ItemIterator = SectionIter<'a>;
    fn get(&'a self, name: &str) -> Option<Self::Item> {
        self.sections.iter().find(|x| x.name == name).cloned()
    }
    fn get_many(&'a self, name: &'a str) -> Self::ItemIterator {
        self.iter(Some(name))
    }
    fn add(&'a self, item: Self::Item) -> Self {
        let mut v = Vec::with_capacity(self.sections.len() + 1);
        v.extend(self.sections.clone());
        v.push(item);
        Sections { sections: v }
    }
    fn add_many(&'a self, items: impl Iterator<Item = Self::Item>) -> Self {
        let mut v = Vec::with_capacity(self.sections.len() + 1);
        v.extend(self.sections.clone());
        v.extend(items);
        Sections { sections: v }
    }
}

impl<'a> Sections<'a> {
    pub fn iter(&self, filter: Option<&'a str>) -> SectionIter {
        SectionIter {
            sections: self.sections.iter(),
            filter,
        }
    }

    pub fn new(s: &'a str) -> Self {
        let mut lines = s.lines().peekable();
        let mut sections: Vec<Section<'a>> = vec![];
        while lines.peek().is_some() {
            let name_line = lines.next().unwrap();
            let mut lines_: Vec<Line<'a>> = vec![];
            loop {
                let line = lines.peek();
                let line = match line {
                    Some(x) => x,
                    None => break,
                };
                if Section::parse_section_name(line).is_some() {
                    break;
                }
                if let Ok(line) = Line::parse(line) {
                    lines_.push(line)
                }
                lines.next();
            }
            let name =
                Section::parse_section_name(name_line).expect("Parser failed or data broken.");
            sections.push(Section::new(name, lines_))
        }
        Self { sections }
    }
}



pub struct SectionIter<'a> {
    sections: std::slice::Iter<'a, Section<'a>>,
    filter: Option<&'a str>,
}

impl<'a> Iterator for SectionIter<'a> {
    type Item = Section<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.sections
            .find(|x| {
                if let Some(f) = self.filter {
                    x.name == f
                } else {
                    true
                }
            })
            .cloned()
    }
}

impl<'a> SectionIter<'a> {}


#[derive(Debug, PartialEq, Clone)]
pub struct Section<'a> {
    pub name: &'a str,
    pub lines: Vec<Line<'a>>,
}

impl<'a> ObjectSelector<'a> for Section<'a> {
    type Item = Line<'a>;
    type ItemIterator = LinesIter<'a>;
    fn get(&'a self, name: &str) -> Option<Self::Item> {
        self.iter(None).find(|x| x.name == name)
    }
    fn get_many(&'a self, name: &'a str) -> Self::ItemIterator {
        self.iter(Some(name))
    }
    fn add(&'a self, item: Self::Item) -> Self {
        let mut v = Vec::with_capacity(self.lines.len() + 1);
        v.extend(self.lines.clone());
        v.push(item);
        Section {
            lines: v,
            name: self.name,
        }
    }
    fn add_many(&'a self, items: impl Iterator<Item = Self::Item>) -> Self {
        let mut v = Vec::with_capacity(self.lines.len() + 1);
        v.extend(self.lines.clone());
        v.extend(items);
        Section {
            lines: v,
            name: self.name,
        }
    }
}

impl<'a> Section<'a> {
    pub fn new(name: &'a str, lines: Vec<Line<'a>>) -> Self {
        Self { name, lines }
    }

    fn parse_section_name(line: &str) -> Option<&str> {
        line.strip_prefix('[').and_then(|x| x.strip_suffix(']'))
    }
    
    fn iter(&'a self, filter: Option<&'a str>) -> LinesIter<'a> {
        LinesIter {
            lines: self.lines.iter(),
            filter,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Line<'a> {
    pub name: &'a str,
    pub value: &'a str
}

impl<'a> Line<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
    
    pub fn with_format(&'a self, format: Line<'a>) -> FormattedLine<'a> {
        FormattedLine::parse(self.name, self.value, format.value)
    }

    pub fn parse(s: &'a str) -> Result<Self, NotAValidLine> {
        let parts = s.splitn(2, ": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(NotAValidLine)
        } else {
            Ok(Self::new(parts[0], parts[1]))
        }
    }
}

pub struct NotAValidLine;

pub struct FormattedLine<'a> {
    name: &'a str,
    fields: Vec<Field<'a>>
}

impl<'a> ObjectSelector<'a> for FormattedLine<'a> {
    type Item = Field<'a>;
    type ItemIterator = FieldIter<'a>;
    
    fn get(&'a self, name: &str) -> Option<Self::Item> {
        self.fields.iter().find(|x| x.name == name).cloned()
    }
    fn get_many(&'a self, name: &'a str) -> Self::ItemIterator {
        FieldIter {
            fields: self.fields.iter(),
            filter: Some(name)
        }
    }
    fn add(&'a self, item: Self::Item) -> Self {
        let mut v = Vec::with_capacity(self.fields.len() + 1);
        v.extend(self.fields.clone());
        v.push(item);
        Self {
            fields: v,
            name: self.name,
        }
    }
    fn add_many(&'a self, items: impl Iterator<Item = Self::Item>) -> Self {
        let mut v = Vec::with_capacity(self.fields.len() + 1);
        v.extend(self.fields.clone());
        v.extend(items);
        Self {
            fields: v,
            name: self.name,
        }
    }
}

impl<'a> FormattedLine<'a> {
    fn parse(name: &'a str, value: &'a str, format: &'a str) -> Self {
        let fields: Vec<_> = format.split(',').map(|x| x.trim()).zip(value.split(',').map(|x| x.trim())).map(|(x, y)| Field {name: x, value: y}).collect();
        Self { name, fields }
    }
}

pub struct FieldIter<'a> {
    fields: std::slice::Iter<'a, Field<'a>>,
    filter: Option<&'a str>,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = Field<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.fields
            .find(|x| {
                if let Some(f) = self.filter {
                    x.name == f
                } else {
                    true
                }
            })
            .cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub value: &'a str
}


pub struct LinesIter<'a> {
    lines: std::slice::Iter<'a, Line<'a>>,
    filter: Option<&'a str>,
}

impl<'a> Iterator for LinesIter<'a> {
    type Item = Line<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines
            .find(|x| {
                if let Some(f) = self.filter {
                    x.name == f
                } else {
                    true
                }
            })
            .cloned()
    }
}






