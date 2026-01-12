use syn::{Ident, LitStr, Token};

use crate::xml::ns::NsSection;

pub struct Tag {
    pub ns: Option<Ns>,
    pub tag: Ident,
}

#[derive(Clone)]
pub struct Ns {
    pub prefix: Ident,
    pub uri: LitStr,
}

impl Tag {
    pub fn try_new(input: syn::parse::ParseStream, ns_section: &NsSection) -> syn::Result<Self> {
        let id: Ident = input.parse()?;
        let (ns, tag) = match input.peek(Token![:]) {
            true => {
                let uri = ns_section.try_lookup(&id)?;
                input.parse::<Token![:]>()?;
                (Some(Ns { prefix: id, uri }), input.parse()?)
            }
            false => (None, id),
        };

        Ok(Self { ns, tag })
    }
}
