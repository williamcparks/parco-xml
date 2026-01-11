use crate::{
    common::ToLitStr,
    dexml::{
        raw::{RawAttr, RawAttrValue},
        visits::VisitAttrs,
    },
};

impl VisitAttrs {
    pub fn new_opt(list: &[RawAttr]) -> Option<Self> {
        if list.is_empty() {
            return None;
        }

        let const_attrs = list
            .iter()
            .filter_map(|raw_attr| match &raw_attr.value {
                RawAttrValue::Const(v) => Some((raw_attr.name.to_lit_str(), v.clone())),
                _ => None,
            })
            .collect();

        let dynamic_attrs = list
            .iter()
            .filter_map(|raw_attr| match &raw_attr.value {
                RawAttrValue::Dynamic(id) => Some((raw_attr.name.to_lit_str(), id.clone())),
                _ => None,
            })
            .collect();

        Some(Self {
            const_attrs,
            dynamic_attrs,
        })
    }
}
