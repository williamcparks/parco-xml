use proc_macro2::TokenStream;

use crate::dexml::visits::Visit;

impl Visit {
    pub fn optimize(list: Vec<Self>) -> Vec<TokenStream> {
        let mut out = Vec::new();

        for i in 0..list.len() {
            let visit = &list[i];
            let rest = &list[i..];

            if !rest.iter().any(Self::requires_work) {
                break;
            }
            out.push(visit.print());
        }

        out
    }
}
