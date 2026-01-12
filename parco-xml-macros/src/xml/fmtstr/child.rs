use crate::xml::c14n::C14nChild;

impl C14nChild {
    pub fn fmtstr(&self, buf: &mut String) {
        match self {
            Self::Dynamic(_) => buf.push_str("{}"),
            Self::Element(el) => el.fmtstr(buf),
        }
    }
}
