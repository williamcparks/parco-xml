use std::borrow::Cow;

use crate::{
    DeXmlError,
    de::Reader,
    dexml::{
        lex::{CloseAngle, Slash},
        parse::Action,
    },
};

/// the ending tag of an element i.e "\>" in "\<Element\>"
pub enum TagEnd {
    /// A self closing tag ending with \/\>. No children
    SelfClosing,
    /// A normal closing tag \>. Potential children
    Normal,
}

impl<'a> Reader<'a> {
    /// parse the ending of a tag, if there are attributes it will consume them and then the end of the element
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

    /// assert this element IS NOT a [`TagEnd::SelfClosing`]
    pub fn assert_children(&mut self) -> Result<(), DeXmlError> {
        match self.tag_end()? {
            TagEnd::Normal => Ok(()),
            TagEnd::SelfClosing => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_CHILDREN))),
        }
    }

    /// if the element has a [`TagEnd::SelfClosing`] does nothing
    /// if the element has a [`TagEnd::Normal`] attempts to parse the closing "\<\/Element\>"
    ///
    /// some elements end in self closing tags such as: "\<Element \/\>"
    /// while others look like: "\<Element\>\<\/Element\>"
    /// [`Reader::tag_close_infer`] can be used in combination with the output of [`Reader::tag_end`]
    /// to parse these correctly
    ///
    /// e.x:
    ///
    /// ```rust,ignore
    /// let tag_end = reader.tag_end()?;
    /// // if the element was self closing it does nothing, else parse the closing tag
    /// reader.tag_close_infer(tag_end)?;
    /// ```
    pub fn tag_close_infer(&mut self, tag_end: TagEnd) -> Result<(), DeXmlError> {
        match tag_end {
            TagEnd::SelfClosing => Ok(()),
            TagEnd::Normal => self.close_tag(),
        }
    }
}
