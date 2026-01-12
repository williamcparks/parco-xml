use std::cmp::Ordering;

use crate::xml::raw::RawAttr;

impl Eq for RawAttr {}

impl Ord for RawAttr {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_ns = self.name.ns.as_ref();
        let b_ns = other.name.ns.as_ref();

        match (a_ns, b_ns) {
            // 1. No-namespace attributes come first
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,

            // 2. Both no namespace → sort by local name
            (None, None) => self.name.tag.cmp(&other.name.tag),

            // 3. Both have namespace → compare namespace URI
            (Some(a_ns), Some(b_ns)) => {
                let a_uri = a_ns.uri.value();
                let b_uri = b_ns.uri.value();

                let ns_cmp = a_uri.cmp(&b_uri);
                if ns_cmp != Ordering::Equal {
                    return ns_cmp;
                }

                self.name.tag.cmp(&other.name.tag)
            }
        }
    }
}

impl PartialEq for RawAttr {
    fn eq(&self, other: &Self) -> bool {
        self.name.tag == other.name.tag
            && self.name.ns.as_ref().map(|v| v.uri.value())
                == other.name.ns.as_ref().map(|v| v.uri.value())
    }
}

impl PartialOrd for RawAttr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
