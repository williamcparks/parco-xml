use std::borrow::Cow;

use crate::{
    DeXml, DeXmlError,
    de::{AppendPath, Reader},
};

macro_rules! num {
    ($ty: ty, $label: literal) => {
        impl<'de> DeXml<'de> for $ty {
            fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
                reader.append_path(AppendPath::Custom($label));
                let text = reader.text()?;
                let value = text
                    .parse::<$ty>()
                    .map_err(|err| reader.err(Cow::Owned(err.to_string())))?;
                reader.exit_path();
                Ok(value)
            }
        }
    };
}

num!(i8, "i8");
num!(i16, "i16");
num!(i32, "i32");
num!(i64, "i64");
num!(isize, "isize");

num!(u8, "u8");
num!(u16, "u16");
num!(u32, "u32");
num!(u64, "u64");
num!(usize, "usize");

num!(f32, "f32");
num!(f64, "f64");

impl<'de> DeXml<'de> for &'de str {
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
        reader.append_path(AppendPath::Text);
        let text = reader.text()?;
        reader.exit_path();
        Ok(text.trim())
    }
}

impl<'de> DeXml<'de> for bool {
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
        reader.append_path(AppendPath::Custom("bool"));
        let text = reader.text()?.trim();
        let res = if text.eq_ignore_ascii_case("true") || text == "1" {
            true
        } else if text.eq_ignore_ascii_case("false") || text == "0" {
            false
        } else {
            return Err(reader.err(Cow::Owned(format!(
                "Expected Boolean Such As `true` Or `false` Saw: `{text}`"
            ))));
        };
        reader.exit_path();
        Ok(res)
    }
}

impl<'de> DeXml<'de> for String {
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
        <&str>::dexml_reader(reader).map(str::to_owned)
    }
}

impl<'de> DeXml<'de> for Box<str> {
    fn dexml_reader(reader: &mut Reader<'de>) -> Result<Self, DeXmlError> {
        <&str>::dexml_reader(reader).map(Into::into)
    }
}
