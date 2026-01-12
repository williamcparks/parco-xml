use proc_macro2::TokenStream;

use crate::xml::{
    c14n::element::C14nElement,
    raw::{Ns, RawChild},
};

pub enum C14nChild {
    Dynamic(TokenStream),
    Element(C14nElement),
}

impl C14nChild {
    pub fn new(child: RawChild, parent_declared_ns: &[Ns]) -> Self {
        match child {
            RawChild::Dynamic(el) => Self::Dynamic(el),
            RawChild::Element(el) => Self::Element(el.c14n(parent_declared_ns)),
        }
    }
}
