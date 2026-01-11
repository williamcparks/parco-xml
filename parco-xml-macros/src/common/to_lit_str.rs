use syn::{Ident, LitStr};

pub trait ToLitStr {
    fn to_lit_str(&self) -> LitStr;
}

impl ToLitStr for Ident {
    fn to_lit_str(&self) -> LitStr {
        LitStr::new(self.to_string().as_str(), self.span())
    }
}
