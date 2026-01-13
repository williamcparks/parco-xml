use crate::{
    DeXmlError,
    de::{PeekedTag, Reader, TagEnd},
};

impl<'a> Reader<'a> {
    /// parse elements till you find the tag, once you do parse it
    pub fn till_open_tag(&mut self, tag: &str) -> Result<(), DeXmlError> {
        while let Some(current_tag) = self.peek_tag() {
            if current_tag == tag {
                break;
            }
            self.block(current_tag)?;
        }

        self.open_tag(tag)
    }

    /// parse elements till you find the ending tag, once you do parse it
    pub fn till_close_tag(&mut self, tag: &str) -> Result<(), DeXmlError> {
        loop {
            match self.peek_open_or_closing_tag() {
                Some(PeekedTag::Open(open)) => self.block(open)?,
                Some(PeekedTag::Close(close)) => {
                    self.close_tag()?;

                    if close == tag {
                        return Ok(());
                    }
                }
                None => {
                    self.text()?;
                }
            };
        }
    }

    /// parse a block i.e "\<Element\>" to "\<\/Element>"
    fn block(&mut self, tag: &str) -> Result<(), DeXmlError> {
        self.open_tag(tag)?;
        let tag_end = self.tag_end()?;

        match tag_end {
            TagEnd::Normal => self.till_close_tag(tag),
            TagEnd::SelfClosing => Ok(()),
        }
    }
}
