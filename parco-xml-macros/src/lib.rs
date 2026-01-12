use proc_macro::TokenStream;
use syn::parse_macro_input;

mod common;
mod dexml;
mod xml;

/// Derives Deserialization Logic Via Custom DSL Templating
///
/// ## Borrow Syntax
///
/// ```rust,ignore
/// // only one lifetime is allowed when borrowing data
/// struct MyStruct<'a> {
///     field: &'a str,
///     captured_attr: &'a str,
/// }
///
/// dexml! {
///     // the ref keyword is for borrowing data
///     ref MyStruct;
///
///     Response {
///         // force an attribute to be a certain value or capture it
///         Field id="required_id" other=(captured_attr) {
///             // capture named fields via parens
///             (field)
///         }
///     }
/// }
/// ```
///
/// ## Owned Syntax
///
/// ```rust,ignore
/// dexml! {
///     // the use keyword tells the macro you don't need a lifetime on your impl
///     use MyStruct;
///
///     // your templating logic here
/// }
/// ```
#[proc_macro]
pub fn dexml(input: TokenStream) -> TokenStream {
    common::out(dexml::handler(parse_macro_input!(input)))
}

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
