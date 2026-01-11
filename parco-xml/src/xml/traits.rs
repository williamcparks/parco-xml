use std::fmt::Formatter;

pub trait Xml: Sized {
    fn xml(&self) -> String {
        self.display().to_string()
    }

    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result;

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
