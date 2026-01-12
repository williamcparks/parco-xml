use std::collections::HashSet;

use syn::Ident;

use crate::xml::raw::{RawChild, RawElement};

impl RawElement {
    pub fn used_namespaces(&self) -> HashSet<&Ident> {
        let mut set = HashSet::new();

        if let Some(ns) = self.tag.ns.as_ref() {
            set.insert(&ns.prefix);
        }
        for attr in self.attrs.iter() {
            if let Some(ns) = attr.name.ns.as_ref() {
                set.insert(&ns.prefix);
            }
        }
        for child in self.children.iter() {
            if let RawChild::Element(el) = child {
                set.extend(el.used_namespaces());
            }
        }

        set
    }
}
