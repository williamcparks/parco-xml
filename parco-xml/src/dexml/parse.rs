use std::borrow::Cow;

use crate::{
    DeXmlError,
    dexml::{
        lex::{LexerError, Token},
        reader::Reader,
    },
};

#[derive(Clone, Copy)]
pub enum Action {
    AttrName,
    AttrValue,

    OpenTag,
    TagEnd,
    CloseTag,

    Text,
}

impl<'a> Reader<'a> {
    pub(crate) fn parse<T: Token<'a>>(&mut self, action: Action) -> Result<T, DeXmlError> {
        match self.lexer.token::<T>() {
            Ok(ok) => Ok(ok),
            Err(err) => match (err, action) {
                (LexerError::Slice(slice), _) => {
                    Err(self.err(Cow::Owned(DeXmlError::slice(slice))))
                }
                (_, Action::AttrName) => {
                    Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ATTR_NAME)))
                }
                (LexerError::ExpectedEqual, Action::AttrValue) => {
                    Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ATTR_EQUAL)))
                }
                (_, Action::AttrValue) => {
                    Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ATTR_VALUE)))
                }
                (_, Action::OpenTag) => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ELEMENT))),
                (_, Action::TagEnd) => {
                    Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_END_OF_ELEMENT)))
                }
                (_, Action::CloseTag) => {
                    Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_CLOSING_ELEMENT)))
                }
                (_, Action::Text) => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_TEXT))),
            },
        }
    }
}
