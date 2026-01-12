use crate::de::Reader;

/// used to control the `path` parameter in error handling
///
/// you should use [`Reader::append_path`] and [`Reader::exit_path`] to
/// control "where" you are in the xml tree so if you encounter an error
/// you can give a helpful message to the user
pub enum AppendPath<'a> {
    /// text node shown in the path as "\:text"
    Text,
    /// attribute shown in the path as "\:attr"
    GeneralAttr,
    /// attribute shown in the path as "\:\[attr_name\]" where the name is [str] you provide
    NamedAttr(&'a str),
    /// element shown in the path as "\/\[element_name\]" where the name is [str] you provide
    Element(&'a str),
    /// custom path show as "\:\[name\]" where the name is [str] you provide
    Custom(&'a str),
}

impl<'a> Reader<'a> {
    /// used to control the `path` parameter in error handling
    ///
    /// you should use [`Reader::append_path`] and [`Reader::exit_path`] to
    /// control "where" you are in the xml tree so if you encounter an error
    /// you can give a helpful message to the user
    pub fn append_path(&mut self, append_path: AppendPath) {
        let (ch, n) = match append_path {
            AppendPath::Text => (':', "text"),
            AppendPath::GeneralAttr => (':', "attr"),
            AppendPath::NamedAttr(attr) => (':', attr),
            AppendPath::Element(el) => ('/', el),
            AppendPath::Custom(el) => (':', el),
        };
        self.path.push(ch);
        self.path.push_str(n);
    }

    /// remove the last part of the path until ":" or "/" for example
    ///
    /// if the path was "/document/element:my_attr"
    /// and [`Reader::exit_path`] is called
    /// then the result is "/document/element"
    ///
    /// ```rust,ignore
    /// reader.append_path(AppendPath::Element("document"));
    /// reader.append_path(AppendPath::Element("element"));
    /// reader.append_path(AppendPath::NamedAttr("my_attr"));
    /// assert_eq!(reader.path(), "/document/element:my_attr");
    ///
    /// reader.exit_path();
    /// assert_eq!(reader.path(), "/document/element");
    ///
    /// ```
    ///
    pub fn exit_path(&mut self) {
        while let Some(ch) = self.path.pop() {
            if matches!(ch, '/' | ':') {
                return;
            }
        }
    }
}
