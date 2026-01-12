use crate::{
    DeXmlError,
    de::{AppendPath, Reader},
    dexml::{
        lex::{Equal, Str},
        parse::Action,
        tag::Tag,
    },
};

impl<'a> Reader<'a> {
    /// deserialize a key value attribute and error if there isn't one or is malformed
    ///
    /// if you want to parse the attr value into a type look at [`Reader::dexml`]
    pub fn attr(&mut self) -> Result<(&'a str, &'a str), DeXmlError> {
        self.append_path(AppendPath::GeneralAttr);

        let Tag(attr_name) = self.parse_tag(Action::AttrName)?;

        self.exit_path();

        self.append_path(AppendPath::NamedAttr(attr_name));

        self.parse::<Equal>(Action::AttrValue)?;
        let Str(attr_value) = self.parse::<Str>(Action::AttrValue)?;
        let attr_value = attr_value.trim_matches(['"', '\'']);

        self.exit_path();

        Ok((attr_name, attr_value))
    }

    /// deserialize a key value attribute and if there isn't an attribute return [`None`]
    ///
    /// if you want to parse the attr value into a type look at [`Reader::dexml`]
    pub fn attr_opt(&mut self) -> Result<Option<(&'a str, &'a str)>, DeXmlError> {
        if self.lexer.peek('/') || self.lexer.peek('>') {
            return Ok(None);
        }
        self.attr().map(Some)
    }
}
