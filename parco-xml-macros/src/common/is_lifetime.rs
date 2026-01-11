use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Lifetime, Token, parse::Parse};

pub struct IsLifetime(pub bool);

impl Parse for IsLifetime {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let res = match input.peek(Token![ref]) {
            true => {
                input.parse::<Token![ref]>()?;
                true
            }
            false => {
                input.parse::<Token![use]>()?;
                false
            }
        };

        Ok(Self(res))
    }
}

impl IsLifetime {
    pub fn print(&self, name: &str) -> TokenStream {
        match self.0 {
            true => {
                let lt = Lifetime::new(name, Span::call_site());
                quote! { <#lt> }
            }
            false => TokenStream::new(),
        }
    }
}
