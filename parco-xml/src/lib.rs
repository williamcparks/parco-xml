#![doc = include_str!("../README.md")]

mod dexml;
mod xml;

pub use dexml::{DeXml, DeXmlError};
pub use parco_xml_macros::{dexml, xml};
pub use xml::Xml;

pub mod de {
    #![doc = "useful deserialization types"]

    pub use crate::dexml::{AppendPath, PeekedTag, Reader, TagEnd};
}

pub mod ser {
    pub use crate::xml::{Conditional, Empty, List};
}
