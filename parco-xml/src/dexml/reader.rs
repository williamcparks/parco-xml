use std::borrow::Cow;

use crate::{
    DeXml, DeXmlError,
    de::AppendPath,
    dexml::{
        lex::{CloseAngle, Lexer, OpenAngle, Slash, Text},
        parse::Action,
    },
};

/// controls XML lexing and parsing
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    pub(super) lexer: Lexer<'a>,
    pub(super) path: String,
}

impl<'a> Reader<'a> {
    /// construct a new lexer from the xml input string and start an empty `path`
    pub const fn new(xml: &'a str) -> Self {
        Self {
            lexer: Lexer::new(xml),
            path: String::new(),
        }
    }

    /// get the path for debugging where you are in the xml tree
    pub const fn path(&self) -> &str {
        self.path.as_str()
    }

    /// attempt to parse "\<\[TAG\]" where TAG is your input, for example `<soap:Envelope` can be parsed via `Reader::open_tag(..., "Envelope")` from the xml input string
    pub fn open_tag(&mut self, tag: &str) -> Result<(), DeXmlError> {
        self.append_path(AppendPath::Element(tag));

        self.parse::<OpenAngle>(Action::OpenTag)?;
        let tag_found = self.parse_tag(Action::OpenTag)?;

        match tag_found.0 == tag {
            true => Ok(()),
            false => Err(self.err(Cow::Borrowed(DeXmlError::EXPECTED_ELEMENT))),
        }
    }

    /// attempt to close a tag via "\<\/TAG\>" doesn't check if the tag is specific name
    pub fn close_tag(&mut self) -> Result<(), DeXmlError> {
        self.parse::<OpenAngle>(Action::CloseTag)?;
        self.parse::<Slash>(Action::CloseTag)?;
        self.parse_tag(Action::CloseTag)?;
        self.parse::<CloseAngle>(Action::CloseTag)?;
        self.exit_path();

        Ok(())
    }

    /// attempt to grab a text node from the xml
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

    /// return an error with the current path and xml input src. The message can be owned or static borrow
    pub fn err(&mut self, message: Cow<'static, str>) -> DeXmlError {
        DeXmlError {
            message,
            path: std::mem::take(&mut self.path),
            xml: self.lexer.xml().into(),
        }
    }

    /// `visit` a type that impls [`DeXml`] if you have a custom type this is where it is parsed and deserialized
    ///
    /// if you are needing to parse a sub document see [`Reader::dexml`]
    pub fn visit<T: DeXml<'a>>(&mut self) -> Result<T, DeXmlError> {
        T::dexml_reader(self)
    }

    /// similar to [`Reader::visit`] but used for attribute parsing for example
    ///
    /// when you call [`Reader::attr`] or [`Reader::attr_opt`] you will end up with the attribute value as a [str] slice
    /// and you may want to parse it into a custom type
    ///
    /// of course if this parsing fails, you want some error logic to show the current path of the parser and where it is the xml tree
    /// for debugging. this method does exactly that, provides a new lexer and reader and parses the [str] as if it were a different document
    /// but moves over it's path, after parsing is complete takes back the path into the crrent reader
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
