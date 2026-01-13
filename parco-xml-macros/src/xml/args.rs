use proc_macro2::TokenStream;

use crate::xml::{
    c14n::{C14nChild, C14nElement},
    raw::RawAttrValue,
};

impl C14nElement {
    pub fn args(&self) -> Vec<TokenStream> {
        let mut out = Vec::new();

        for attr in self.attrs.iter() {
            if let RawAttrValue::Dynamic(el) = &attr.value {
                out.push(el.clone());
            }
        }

        for child in self.children.iter() {
            match child {
                C14nChild::Dynamic(el) => out.push(el.clone()),
                C14nChild::Const(_) => {}
                C14nChild::Element(el) => out.extend(el.args()),
            }
        }

        out
    }
}
