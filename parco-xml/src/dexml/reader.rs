use std::borrow::Cow;

use crate::{
    DeXml, DeXmlError,
    de::AppendPath,
    dexml::{
        lex::{CloseAngle, Lexer, OpenAngle, Slash, Text},
        parse::Action,
    },
};

#[derive(Clone, Debug)]
pub struct Reader<'a> {
    pub(super) lexer: Lexer<'a>,
    pub(super) path: String,
}

impl<'a> Reader<'a> {
    pub const fn new(xml: &'a str) -> Self {
        Self {
            lexer: Lexer::new(xml),
            path: String::new(),
        }
    }

    pub fn open_tag(&mut self, tag: &str) -> Result<(), DeXmlError> {
        self.append_path(AppendPath::Element(tag));

        self.parse::<OpenAngle>(Action::OpenTag)?;
        let tag_found = self.parse_tag(Action::OpenTag)?;

        match tag_found.0 == tag {
            true => Ok(()),
            false => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ELEMENT))),
        }
    }

    pub fn close_tag(&mut self) -> Result<(), DeXmlError> {
        self.parse::<OpenAngle>(Action::CloseTag)?;
        self.parse::<Slash>(Action::CloseTag)?;
        self.parse_tag(Action::CloseTag)?;
        self.parse::<CloseAngle>(Action::CloseTag)?;
        self.exit_path();
        Ok(())
    }

    pub fn text(&mut self) -> Result<&'a str, DeXmlError> {
        self.append_path(AppendPath::Text);

        match self.parse::<Text>(Action::Text) {
            Ok(Text(text)) => {
                self.exit_path();
                Ok(text.trim())
            }
            Err(err) => Err(err),
        }
    }

    pub fn err(&mut self, message: Cow<'static, str>) -> DeXmlError {
        DeXmlError {
            message,
            path: std::mem::take(&mut self.path),
            xml: self.lexer.xml().into(),
        }
    }

    pub fn visit<T: DeXml<'a>>(&mut self) -> Result<T, DeXmlError> {
        T::dexml_reader(self)
    }

    pub fn dexml<T: DeXml<'a>>(&mut self, arg: &'a str) -> Result<T, DeXmlError> {
        let mut sub_reader = Self {
            lexer: Lexer::new(arg),
            path: ::std::mem::take(&mut self.path),
        };
        let res = T::dexml_reader(&mut sub_reader)?;
        self.path = sub_reader.path;
        Ok(res)
    }
}
