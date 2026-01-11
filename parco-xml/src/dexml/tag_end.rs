use std::borrow::Cow;

use crate::{
    DeXmlError,
    de::Reader,
    dexml::{
        lex::{CloseAngle, Slash},
        parse::Action,
    },
};

pub enum TagEnd {
    /// A self closing tag ending with />. No children
    SelfClosing,
    /// A normal closing tag >. Potential children
    Normal,
}

impl<'a> Reader<'a> {
    pub fn tag_end(&mut self) -> Result<TagEnd, DeXmlError> {
        loop {
            if self.lexer.peek('/') {
                self.parse::<Slash>(Action::TagEnd)?;
                self.parse::<CloseAngle>(Action::TagEnd)?;
                return Ok(TagEnd::SelfClosing);
            }
            if self.lexer.peek('>') {
                self.parse::<CloseAngle>(Action::TagEnd)?;
                return Ok(TagEnd::Normal);
            }
            self.attr()?;
        }
    }

    pub fn assert_children(&mut self) -> Result<(), DeXmlError> {
        match self.tag_end()? {
            TagEnd::Normal => Ok(()),
            TagEnd::SelfClosing => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_CHILDREN))),
        }
    }

    pub fn tag_close_infer(&mut self, tag_end: TagEnd) -> Result<(), DeXmlError> {
        match tag_end {
            TagEnd::SelfClosing => Ok(()),
            TagEnd::Normal => self.close_tag(),
        }
    }
}
