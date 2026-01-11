use syn::{
    Ident, LitStr, Token, parenthesized,
    parse::Parse,
    token::{Brace, Paren},
};

pub struct RawAttr {
    pub name: Ident,
    pub value: RawAttrValue,
}

pub enum RawAttrValue {
    Const(LitStr),
    Dynamic(Ident),
}

impl RawAttr {
    pub fn parse_many(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = Vec::new();

        while !input.peek(Brace) {
            attrs.push(input.parse()?);
        }

        Ok(attrs)
    }
}

impl Parse for RawAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;

        Ok(Self {
            name,
            value: input.parse()?,
        })
    }
}

impl Parse for RawAttrValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.peek(Paren) {
            true => {
                let section;
                parenthesized!(section in input);
                Ok(Self::Dynamic(section.parse()?))
            }
            false => Ok(Self::Const(input.parse()?)),
        }
    }
}
