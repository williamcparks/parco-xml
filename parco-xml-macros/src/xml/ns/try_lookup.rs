use syn::{Error, Ident, LitStr, Result};

use super::NsSection;

impl NsSection {
    pub fn try_lookup(&self, id: &Ident) -> Result<LitStr> {
        match self.0.get(id) {
            Some(some) => Ok(some.clone()),
            None => Err(Error::new(
                id.span(),
                format!("Namespace Prefix `{id}` Was Not Declared In @ns Section"),
            )),
        }
    }
}
