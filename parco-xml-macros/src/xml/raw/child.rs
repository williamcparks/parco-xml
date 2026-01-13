use proc_macro2::TokenStream;
use syn::{LitStr, braced, parenthesized, token::Paren};

use crate::xml::{ns::NsSection, raw::RawElement};

pub enum RawChild {
    Dynamic(TokenStream),
    Const(LitStr),
    Element(RawElement),
}

impl RawChild {
    pub fn try_new(input: syn::parse::ParseStream, ns_section: &NsSection) -> syn::Result<Self> {
        if input.peek(Paren) {
            let section;
            parenthesized!(section in input);
            Ok(Self::Dynamic(section.parse()?))
        } else if input.peek(LitStr) {
            Ok(Self::Const(input.parse()?))
        } else {
            Ok(Self::Element(RawElement::try_new(input, ns_section)?))
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
