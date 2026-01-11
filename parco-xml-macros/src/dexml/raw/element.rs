use syn::{Ident, parse::Parse};

use crate::dexml::raw::{RawAttr, RawChild};

pub struct RawElement {
    pub tag: Ident,
    pub attrs: Vec<RawAttr>,
    pub children: Vec<RawChild>,
}

impl Parse for RawElement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tag: input.parse()?,
            attrs: RawAttr::parse_many(input)?,
            children: RawChild::parse_many(input)?,
        })
    }
}
