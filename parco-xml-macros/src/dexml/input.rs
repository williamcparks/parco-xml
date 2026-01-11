use syn::{Token, Type, parse::Parse};

use crate::{common::IsLifetime, dexml::raw::RawElement};

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

        Ok(Self {
            is_lifetime,
            ty,
            element: input.parse()?,
        })
    }
}
