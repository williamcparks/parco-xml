use syn::Ident;

use crate::dexml::raw::{RawAttrValue, RawChild, RawElement};

impl RawElement {
    pub fn inits(&self) -> Vec<Ident> {
        let mut out = Vec::new();

        for attr in self.attrs.iter() {
            if let RawAttrValue::Dynamic(id) = &attr.value {
                out.push(id.clone());
            }
        }
        for child in self.children.iter() {
            match child {
                RawChild::Dynamic(id) => out.push(id.clone()),
                RawChild::Element(el) => out.extend(el.inits()),
            }
        }

        out
    }
}
