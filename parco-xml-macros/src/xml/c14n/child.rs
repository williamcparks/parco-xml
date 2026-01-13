use proc_macro2::TokenStream;
use syn::LitStr;

use crate::xml::{
    c14n::element::C14nElement,
    raw::{Ns, RawChild},
};

pub enum C14nChild {
    Dynamic(TokenStream),
    Const(LitStr),
    Element(C14nElement),
}

impl C14nChild {
    pub fn new(child: RawChild, parent_declared_ns: &[Ns]) -> Self {
        match child {
            RawChild::Dynamic(el) => Self::Dynamic(el),
            RawChild::Const(lit) => Self::Const(lit),
            RawChild::Element(el) => Self::Element(el.c14n(parent_declared_ns)),
        }
    }
}
