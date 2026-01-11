use proc_macro::TokenStream;
use syn::parse_macro_input;

mod common;
mod dexml;
mod xml;

#[proc_macro]
pub fn dexml(input: TokenStream) -> TokenStream {
    common::out(dexml::handler(parse_macro_input!(input)))
}

#[proc_macro]
pub fn xml(input: TokenStream) -> TokenStream {
    common::out(xml::handler(parse_macro_input!(input)))
}
