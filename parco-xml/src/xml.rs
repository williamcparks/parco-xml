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

impl<T: Xml> Xml for &T {
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        T::serialize_xml(self, fmt)
    }
}

impl<T: Xml> Xml for &mut T {
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        T::serialize_xml(self, fmt)
    }
}

impl<T: Xml> Xml for Box<T> {
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        T::serialize_xml(self, fmt)
    }
}

pub struct Display<'a, T>(&'a T);

impl<'a, T: Xml> std::fmt::Display for Display<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::serialize_xml(self.0, f)
    }
}

/// Renders nothing to the xml
pub struct Empty;

impl Xml for Empty {
    fn serialize_xml(&self, _fmt: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Renders T only if Some(T)
pub struct Conditional<T>(pub Option<T>);

impl<T: Xml> Xml for Conditional<T> {
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            Some(some) => some.serialize_xml(fmt),
            None => Ok(()),
        }
    }
}

/// Renders every T inside the [`Vec`]
pub struct List<T>(pub Vec<T>);

impl<T> Xml for List<T>
where
    T: Xml,
{
    fn serialize_xml(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            item.serialize_xml(fmt)?;
        }
        Ok(())
    }
}
