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

    pub fn attr_opt(&mut self) -> Result<Option<(&'a str, &'a str)>, DeXmlError> {
        if self.lexer.peek('/') || self.lexer.peek('>') {
            return Ok(None);
        }
        self.attr().map(Some)
    }
}
