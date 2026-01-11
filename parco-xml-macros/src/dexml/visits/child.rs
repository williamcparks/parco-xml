use crate::dexml::{raw::RawChild, visits::Visit};

impl RawChild {
    pub fn visits(&self) -> Vec<Visit> {
        match self {
            Self::Dynamic(id) => vec![Visit::Vis(id.clone())],
            Self::Element(el) => el.visits(),
        }
    }
}
