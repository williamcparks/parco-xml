use crate::xml::{
    ns::NsSection,
    raw::{RawAttr, RawChild, Tag},
};

pub struct RawElement {
    pub tag: Tag,
    pub attrs: Vec<RawAttr>,
    pub children: Vec<RawChild>,
}

impl RawElement {
    pub fn try_new(input: syn::parse::ParseStream, ns_section: &NsSection) -> syn::Result<Self> {
        Ok(Self {
            tag: Tag::try_new(input, ns_section)?,
            attrs: RawAttr::parse_many(input, ns_section)?,
            children: RawChild::children(input, ns_section)?,
        })
    }
}
