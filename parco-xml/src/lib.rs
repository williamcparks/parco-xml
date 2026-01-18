#![doc = include_str!("../README.md")]

mod xml;

pub use parco_xml_macros::xml;
pub use quick_xml;
pub use xml::Xml;

pub mod ser {
    pub use crate::xml::{Conditional, Empty, List};
}
