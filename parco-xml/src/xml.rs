use std::fmt::Formatter;

/// A trait the allows you to serialize data to xml
pub trait Xml: Sized {
    /// serialize xml to a string
    fn xml(&self) -> String {
        self.display().to_string()
    }

    /// serialize xml to a formatter
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result;

    /// serialize xml via [`std::fmt::Display`] trait
    fn display<'a>(&'a self) -> Display<'a, Self> {
        Display(self)
    }
}

pub struct Display<'a, T>(&'a T);

impl<'a, T: Xml> std::fmt::Display for Display<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::serialize_xml(self.0, f)
    }
}
