use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

use crate::dexml::{input::Input, visits::Visit};

pub fn handler(input: Input) -> Result<TokenStream> {
    let lt = input.is_lifetime.print("'de");
    let user_type = input.ty;

    let visits = Visit::optimize(input.element.visits());
    let inits = input.element.inits();

    Ok(quote! {
        impl<'de> ::parco_xml::DeXml<'de> for #user_type #lt {
            fn dexml_reader(reader: &mut ::parco_xml::de::Reader<'de>) -> ::core::result::Result<Self, ::parco_xml::DeXmlError> {
                #(#visits)*

                ::core::result::Result::Ok(Self {
                    #(#inits),*
                })
            }
        }
    })
}
