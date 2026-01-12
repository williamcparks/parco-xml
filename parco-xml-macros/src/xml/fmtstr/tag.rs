use std::fmt::Write;

use crate::xml::raw::Tag;

impl Tag {
    pub fn fmtstr(&self, buf: &mut String) {
        if let Some(ns) = self.ns.as_ref() {
            let _ = write!(buf, "{}:", ns.prefix);
        }
        let _ = write!(buf, "{}", self.tag);
    }
}
