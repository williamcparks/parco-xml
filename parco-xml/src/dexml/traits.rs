use std::borrow::Cow;

use thiserror::Error;

use crate::de::Reader;

/// a trait the allows types to deserialize themselves from xml
pub trait DeXml<'de>: Sized {
    /// deserialize from a xml source [`str`], automatically implemented for types once [`DeXml::dexml_reader`] is implemented
    fn dexml(xml: &'de str) -> Result<Self, DeXmlError> {
        let mut rdr = Reader::new(xml);
        Self::dexml_reader(&mut rdr)
    }

    /// required method for [`DeXml`] trait. the reader provides useful parsing and lexing primitives for deserializing your type
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError>;
}

/// a error with a `path` from where in xml document the error happened and the entire xml document for reference
/// along with a custom message
#[derive(Debug, Error)]
#[error("{message} @ `{path}`\n\n{xml}")]
pub struct DeXmlError {
    /// the message of what went wrong
    pub message: Cow<'static, str>,
    /// where in the xml document
    pub path: String,
    /// the xml document
    pub xml: Box<str>,
}

impl DeXmlError {
    pub(crate) fn slice(cursor: usize) -> String {
        format!("Failed To Slice XML Source @ Byte: `{cursor}`")
    }

    pub(crate) const EXPECTED_ATTR_NAME: &str = "Expected Attribute Name On Element";
    pub(crate) const EXPECTED_ATTR_EQUAL: &str = "Expected '=' For Attribute";
    pub(crate) const EXPECTED_ATTR_VALUE: &str = "Expected A String '...' For Attribute Value";
    pub(crate) const EXPECTED_ELEMENT: &str = "Expected Element";
    pub(crate) const EXPECTED_END_OF_ELEMENT: &str = "Expected End Of Element";
    pub(crate) const EXPECTED_CLOSING_ELEMENT: &str = "Expected Closing Element";
    pub(crate) const EXPECTED_CHILDREN: &str =
        "Expected Element To Have Children But It Was A Self Closing Element";
    pub(crate) const EXPECTED_TEXT: &str = "Expected Text Node";
}
