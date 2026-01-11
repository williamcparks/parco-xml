use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

use crate::xml::input::Input;

pub fn handler(input: Input) -> Result<TokenStream> {
    let lt = input.is_lifetime.print("'xml");
    let user_type = input.ty;

    Ok(quote! {
        impl #lt ::parco_xml::Xml for #user_type #lt {
            fn serialize_xml(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::core::result::Result::Ok(())
            }
        }
    })
}
