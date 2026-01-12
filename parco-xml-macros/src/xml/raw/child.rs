use proc_macro2::TokenStream;
use syn::{braced, parenthesized, token::Paren};

use crate::xml::{ns::NsSection, raw::RawElement};

pub enum RawChild {
    Dynamic(TokenStream),
    Element(RawElement),
}

impl RawChild {
    pub fn try_new(input: syn::parse::ParseStream, ns_section: &NsSection) -> syn::Result<Self> {
        match input.peek(Paren) {
            true => {
                let section;
                parenthesized!(section in input);
                Ok(Self::Dynamic(section.parse()?))
            }
            false => Ok(Self::Element(RawElement::try_new(input, ns_section)?)),
        }
    }

    pub fn children(
        main: syn::parse::ParseStream,
        ns_section: &NsSection,
    ) -> syn::Result<Vec<Self>> {
        let input;
        braced!(input in main);

        let mut children = Vec::new();
        while !input.is_empty() {
            children.push(Self::try_new(&input, ns_section)?);
        }

        Ok(children)
    }
}
