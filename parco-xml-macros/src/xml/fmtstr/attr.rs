use crate::xml::raw::{RawAttr, RawAttrValue};

impl RawAttr {
    pub fn fmtstr(&self, buf: &mut String) {
        if !buf.ends_with(' ') {
            buf.push(' ');
        }
        self.name.fmtstr(buf);
        buf.push('=');
        buf.push('"');
        self.value.fmtstr(buf);
        buf.push('"');
    }
}

impl RawAttrValue {
    pub fn fmtstr(&self, buf: &mut String) {
        match self {
            Self::Const(c) => buf.push_str(c.value().as_str()),
            Self::Dynamic(_) => buf.push_str("{}"),
        }
    }
}
