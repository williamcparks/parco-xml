use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr};

use crate::dexml::visits::VisitAttrs;

pub enum Visit {
    TillOpenTag(LitStr),
    CloseTag,
    AssertChildren,
    VisitAttrs(VisitAttrs),
    Vis(Ident),
    Block(LitStr),
    TagEndSetup,
    TagCloseInfer(LitStr),
}

impl Visit {
    pub fn print(&self) -> TokenStream {
        match self {
            Self::TillOpenTag(tag) => quote! { reader.till_open_tag(#tag)?; },
            Self::CloseTag => quote! { reader.close_tag()?; },
            Self::AssertChildren => quote! { reader.assert_children()?; },
            Self::VisitAttrs(attrs) => attrs.print(),
            Self::Vis(id) => quote! { let #id = reader.visit()?; },
            Self::Block(block) => quote! { reader.block(#block)?; },
            Self::TagEndSetup => quote! { let tag_end = reader.tag_end()?;  },
            Self::TagCloseInfer(tag) => quote! { reader.till_close_infer_tag(#tag, tag_end)?; },
        }
    }

    pub fn requires_work(&self) -> bool {
        matches!(self, Self::Vis(_) | Self::VisitAttrs(_))
    }
}

impl ToTokens for Visit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.print())
    }
}
