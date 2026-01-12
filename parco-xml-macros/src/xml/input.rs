use syn::{Error, Token, Type, parse::Parse};

use crate::{
    common::IsLifetime,
    xml::{ns::NsSection, raw::RawElement},
};

pub struct Input {
    pub is_lifetime: IsLifetime,
    pub ty: Type,
    pub element: RawElement,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let is_lifetime = input.parse()?;

        let ty = input.parse()?;
        input.parse::<Token![;]>()?;

        let ns_section = input.parse::<NsSection>()?;
        let element = RawElement::try_new(input, &ns_section)?;

        let used = element.used_namespaces();
        let declared = ns_section.declared();
        if let Some(ns) = declared.difference(&used).next() {
            return Err(Error::new(
                ns.span(),
                format!("Namespace `{ns}` Declared But Never Used"),
            ));
        }

        Ok(Self {
            is_lifetime,
            ty,
            element,
        })
    }
}
