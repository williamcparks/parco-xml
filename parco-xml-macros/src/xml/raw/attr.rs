use proc_macro2::TokenStream;
use syn::{LitStr, Token, parenthesized, parse::Parse, token::Brace};

use crate::xml::{ns::NsSection, raw::Tag};

pub struct RawAttr {
    pub name: Tag,
    pub value: RawAttrValue,
}

pub enum RawAttrValue {
    Const(LitStr),
    Dynamic(TokenStream),
}

impl RawAttr {
    pub fn try_new(input: syn::parse::ParseStream, ns_section: &NsSection) -> syn::Result<Self> {
        let name = Tag::try_new(input, ns_section)?;
        input.parse::<Token![=]>()?;

        Ok(Self {
            name,
            value: input.parse()?,
        })
    }

    pub fn parse_many(
        input: syn::parse::ParseStream,
        ns_section: &NsSection,
    ) -> syn::Result<Vec<Self>> {
        let mut out = Vec::new();
        while !input.peek(Brace) {
            out.push(Self::try_new(input, ns_section)?);
        }
        Ok(out)
    }
}

impl Parse for RawAttrValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.peek(LitStr) {
            true => Ok(Self::Const(input.parse()?)),
            false => {
                let section;
                parenthesized!(section in input);
                Ok(Self::Dynamic(section.parse()?))
            }
        }
    }
}
