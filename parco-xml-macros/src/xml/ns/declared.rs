use std::collections::HashSet;

use syn::Ident;

use crate::xml::ns::NsSection;

impl NsSection {
    pub fn declared(&self) -> HashSet<&Ident> {
        self.0.keys().collect()
    }
}
