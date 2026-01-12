use crate::xml::c14n::C14nElement;

impl C14nElement {
    pub fn fmtstr(&self, buf: &mut String) {
        buf.push('<');
        self.tag.fmtstr(buf);

        for ns in self.ns_attrs.iter() {
            ns.fmtstr(buf);
        }

        for attr in self.attrs.iter() {
            attr.fmtstr(buf);
        }

        buf.push('>');
        for child in self.children.iter() {
            child.fmtstr(buf);
        }

        buf.push_str("</");
        self.tag.fmtstr(buf);
        buf.push('>');
    }
}
