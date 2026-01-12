use std::cmp::Ordering;

use crate::xml::raw::{Ns, RawElement};

impl Ns {
    pub fn ns_attrs(element: &RawElement, parent_declared_ns: &[Self]) -> Vec<Self> {
        let mut out = Vec::new();

        if let Some(ns) = element.tag.ns.as_ref()
            && !parent_declared_ns.contains(ns)
        {
            out.push(ns.clone());
        }
        for attr in element.attrs.iter() {
            if let Some(ns) = attr.name.ns.as_ref()
                && !parent_declared_ns.contains(ns)
                && !out.contains(ns)
            {
                out.push(ns.clone());
            }
        }

        out.sort();

        out
    }
}

impl Eq for Ns {}

impl Ord for Ns {
    fn cmp(&self, other: &Self) -> Ordering {
        self.prefix.cmp(&other.prefix)
    }
}

impl PartialEq for Ns {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix
    }
}

impl PartialOrd for Ns {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
