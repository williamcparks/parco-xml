use crate::{
    DeXmlError,
    dexml::{
        lex::{Colon, Ident, OpenAngle, Slash},
        parse::Action,
        reader::Reader,
    },
};

pub struct Tag<'a>(pub &'a str);

/// a peeked tag either Open i.e "\<Element" or "\<\/Element"
pub enum PeekedTag<'a> {
    /// open tag "\<Element"
    Open(&'a str),
    /// close tag "\<\/Element"
    Close(&'a str),
}

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

        lexer.token::<OpenAngle>().ok()?;

        let ident = lexer.token::<Ident>().ok()?;
        if lexer.peek(':') {
            lexer.token::<Colon>().ok()?;
            let id = lexer.token::<Ident>().ok()?;
            return Some(id.0);
        }

        Some(ident.0)
    }

    /// similar to [`Reader::peek_tag`] but inspects the next closing tag i.e "\<\/tag"
    pub fn peek_closing_tag(&mut self) -> Option<&'a str> {
        let mut lexer = self.lexer.clone();

        lexer.token::<OpenAngle>().ok()?;
        lexer.token::<Slash>().ok()?;

        let ident = lexer.token::<Ident>().ok()?;
        if lexer.peek(':') {
            lexer.token::<Colon>().ok()?;
            let id = lexer.token::<Ident>().ok()?;
            return Some(id.0);
        }

        Some(ident.0)
    }

    /// similar to [`Reader::peek_tag`] but accepts either an opening tag or closing tag
    pub fn peek_open_or_closing_tag(&mut self) -> Option<PeekedTag<'a>> {
        let mut lexer = self.lexer.clone();

        lexer.token::<OpenAngle>().ok()?;
        let is_close = match lexer.peek('/') {
            true => {
                lexer.token::<Slash>().ok()?;
                true
            }
            false => false,
        };

        let ident = lexer.token::<Ident>().ok()?;
        let tag = match lexer.peek(':') {
            true => {
                lexer.token::<Colon>().ok()?;
                let id = lexer.token::<Ident>().ok()?;
                id.0
            }
            false => ident.0,
        };

        match is_close {
            true => Some(PeekedTag::Close(tag)),
            false => Some(PeekedTag::Open(tag)),
        }
    }
}
