use crate::{
    common::ToLitStr,
    dexml::{
        raw::RawElement,
        visits::{Visit, VisitAttrs},
    },
};

impl RawElement {
    pub fn visits(&self) -> Vec<Visit> {
        let mut out = vec![Visit::OpenTag(self.tag.to_lit_str())];

        if let Some(visit_attrs) = VisitAttrs::new_opt(self.attrs.as_slice()) {
            out.push(Visit::VisitAttrs(visit_attrs));
        }

        if !self.children.is_empty() {
            out.push(Visit::AssertChildren);
        } else {
            out.push(Visit::TagEndSetup);
        }

        for child in self.children.iter() {
            out.extend(child.visits());
        }

        if !self.children.is_empty() {
            out.push(Visit::CloseTag);
        } else {
            out.push(Visit::TagCloseInfer);
        }

        out
    }
}
