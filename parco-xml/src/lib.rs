mod dexml;
mod xml;

pub use dexml::{DeXml, DeXmlError};
pub use parco_xml_macros::{dexml, xml};
pub use xml::Xml;

pub mod de {
    pub use crate::dexml::{AppendPath, Reader, TagEnd};
}
