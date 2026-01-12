use proc_macro2::TokenStream;
use quote::quote;
use syn::{LitStr, Result};

use crate::xml::input::Input;

pub fn handler(input: Input) -> Result<TokenStream> {
    let lt = input.is_lifetime.print("'xml");
    let user_type = input.ty;

    let c14n = input.element.c14n(&[]);

    let mut fmtstr = String::new();
    c14n.fmtstr(&mut fmtstr);
    let fmtstr = LitStr::new(fmtstr.as_str(), c14n.tag.tag.span());

    let args = c14n.args();

    Ok(quote! {
        #[automatically_derived]
        impl #lt ::parco_xml::Xml for #user_type #lt {
            fn serialize_xml(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use ::std::fmt::Write;

                ::std::write!(f, #fmtstr, #(#args),*)
            }
        }
    })
}
