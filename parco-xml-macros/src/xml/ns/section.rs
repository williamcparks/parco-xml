use std::collections::HashMap;

use syn::{Error, Ident, LitStr, Token, braced, custom_keyword, parse::Parse};

pub struct NsSection(pub HashMap<Ident, LitStr>);

custom_keyword!(ns);

impl Parse for NsSection {
    fn parse(main: syn::parse::ParseStream) -> syn::Result<Self> {
        main.parse::<Token![@]>()?;
        main.parse::<ns>()?;

        let input;
        braced!(input in main);

        let mut map = HashMap::<Ident, LitStr>::new();
        let punct = input.parse_terminated(Pair::parse, Token![,])?;
        for Pair(key, value) in punct {
            if let Some(prev) = map.get(&key) {
                let msg = format!(
                    "Namespace Prefix `{key}` Was Previous Assigned To `{}` But Here It Is Being Assigned To `{}`",
                    prev.value(),
                    value.value()
                );
                return Err(Error::new(key.span(), msg));
            }
            map.insert(key, value);
        }

        Ok(Self(map))
    }
}

struct Pair(Ident, LitStr);

impl Parse for Pair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self(key, value))
    }
}
