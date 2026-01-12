use std::fmt::Write;

use crate::xml::raw::Ns;

impl Ns {
    pub fn fmtstr(&self, buf: &mut String) {
        if !buf.ends_with(' ') {
            buf.push(' ');
        }

        let _ = write!(buf, "xmlns:{}=\"{}\"", self.prefix, self.uri.value());
    }
}
