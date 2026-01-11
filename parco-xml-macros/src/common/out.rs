use proc_macro2::TokenStream;
use syn::Result;

pub fn out(result: Result<TokenStream>) -> proc_macro::TokenStream {
    match result {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
