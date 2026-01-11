use syn::{Ident, braced, parenthesized, parse::Parse, token::Paren};

use crate::dexml::raw::RawElement;

pub enum RawChild {
    Element(RawElement),
    Dynamic(Ident),
}

impl RawChild {
    pub fn parse_many(main: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let input;
        braced!(input in main);

        let mut children = Vec::new();

        while !input.is_empty() {
            children.push(input.parse()?);
        }

        Ok(children)
    }
}

impl Parse for RawChild {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.peek(Paren) {
            true => {
                let section;
                parenthesized!(section in input);
                Ok(Self::Dynamic(section.parse()?))
            }
            false => Ok(Self::Element(input.parse()?)),
        }
    }
}
