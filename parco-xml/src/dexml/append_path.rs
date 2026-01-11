use crate::de::Reader;

pub enum AppendPath<'a> {
    Text,
    GeneralAttr,
    NamedAttr(&'a str),
    Element(&'a str),
    Custom(&'a str),
}

impl<'a> Reader<'a> {
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

    pub fn exit_path(&mut self) {
        while let Some(ch) = self.path.pop() {
            if matches!(ch, '/' | ':') {
                return;
            }
        }
    }
}
