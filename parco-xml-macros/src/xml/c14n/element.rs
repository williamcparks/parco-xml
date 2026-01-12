use crate::xml::{
    c14n::C14nChild,
    raw::{Ns, RawAttr, RawElement, Tag},
};

pub struct C14nElement {
    pub tag: Tag,
    pub ns_attrs: Vec<Ns>,
    pub attrs: Vec<RawAttr>,
    pub children: Vec<C14nChild>,
}

impl RawElement {
    pub fn c14n(mut self, parent_declared_ns: &[Ns]) -> C14nElement {
        self.attrs.sort();

        let ns_attrs = Ns::ns_attrs(&self, parent_declared_ns);
        let mut children = Vec::new();

        let mut merged_ns: Vec<_> = ns_attrs.clone();
        merged_ns.extend(parent_declared_ns.iter().cloned());
        for child in self.children {
            children.push(C14nChild::new(child, merged_ns.as_slice()));
        }

        C14nElement {
            tag: self.tag,
            ns_attrs,
            attrs: self.attrs,
            children,
        }
    }
}
