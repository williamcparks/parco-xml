use proc_macro::TokenStream;
use syn::parse_macro_input;

mod common;
mod xml;

/// Derives Serialization Logic Via Custom DSL Templating
///
/// ## Borrow Syntax
///
/// ```rust,ignore
/// // you are only allowed one lifetime if you are borrowing
/// struct MyStruct<'a> {
///     field: &'a str,
/// }
///
/// xml! {
///     // the keyword ref tells the macro you are borrowing
///     ref MyStruct;
///     
///     // define your namespaces here
///     @ns {
///         myns = "uri:myns",
///     }
///
///     myns:Element attr=(self.field) {
///         // parens allow you to write any expression
///         (self.field)
///     }
/// }
/// ```
///
/// ## Owned Syntax
///
/// ```rust,ignore
///
/// struct MyStruct {
///     owned: String,
/// }
///
/// xml! {
///     // the use keyword means you aren't borrowing data
///     use MyStruct;
///
///     // define your namespaces here
///     @ns {
///         myns = "uri:myns",    
///     }
///
///     myns:Element {
///         (self.owned)
///     }
/// }
/// ```
///
#[proc_macro]
pub fn xml(input: TokenStream) -> TokenStream {
    common::out(xml::handler(parse_macro_input!(input)))
}
