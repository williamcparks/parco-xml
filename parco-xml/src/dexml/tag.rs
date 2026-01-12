use crate::{
    DeXmlError,
    dexml::{
        lex::{Colon, Ident, OpenAngle},
        parse::Action,
        reader::Reader,
    },
};

pub struct Tag<'a>(pub &'a str);

impl<'a> Reader<'a> {
    pub(crate) fn parse_tag(&mut self, action: Action) -> Result<Tag<'a>, DeXmlError> {
        let ident: Ident = self.parse(action)?;

        match self.lexer.peek(':') {
            true => {
                self.parse::<Colon>(action)?;
                let ident: Ident = self.parse(action)?;
                Ok(Tag(ident.0))
            }
            false => Ok(Tag(ident.0)),
        }
    }

    /// inspect the next tag, can be used for parsing [`std::vec::Vec`]
    pub fn peek_tag(&mut self) -> Option<&'a str> {
        let mut lexer = self.lexer.clone();

        if lexer.peek('<') {
            lexer.token::<OpenAngle>().ok()?;
        }
        let ident = lexer.token::<Ident>().ok()?;
        if lexer.peek(':') {
            lexer.token::<Colon>().ok()?;
            let id = lexer.token::<Ident>().ok()?;
            return Some(id.0);
        }

        Some(ident.0)
    }
}
