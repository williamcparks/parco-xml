use std::borrow::Cow;

use thiserror::Error;

use crate::de::Reader;

pub trait DeXml<'de>: Sized {
    fn dexml(xml: &'de str) -> Result<Self, DeXmlError> {
        let mut rdr = Reader::new(xml);
        Self::dexml_reader(&mut rdr)
    }

    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError>;
}

#[derive(Debug, Error)]
#[error("{message} @ `{path}`\n\n{xml}")]
pub struct DeXmlError {
    pub message: Cow<'static, str>,
    pub path: String,
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
