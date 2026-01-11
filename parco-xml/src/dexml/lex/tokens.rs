use super::{Lexer, LexerError, Token};

macro_rules! symbol {
    ($ident: ident, $pat: literal, $id: ident) => {
        impl<'a> Token<'a> for $ident {
            fn token(input: &mut Lexer<'a>) -> Result<Self, LexerError> {
                match input.src()?.starts_with($pat) {
                    true => {
                        input.incr($pat.len_utf8());
                        Ok(Self)
                    }
                    false => Err(LexerError::$id),
                }
            }
        }
    };
}

pub struct OpenAngle;

symbol!(OpenAngle, '<', ExpectedOpenAngle);

pub struct CloseAngle;

symbol!(CloseAngle, '>', ExpectedCloseAngle);

pub struct Slash;

symbol!(Slash, '/', ExpectedSlash);

pub struct Equal;

symbol!(Equal, '=', ExpectedEqual);

pub struct Colon;

symbol!(Colon, ':', ExpectedColon);

pub struct Ident<'a>(pub &'a str);

impl<'a> Token<'a> for Ident<'a> {
    fn token(input: &mut Lexer<'a>) -> Result<Self, LexerError> {
        let src = input.src()?;

        match super::ident::ident(src) {
            Some(some) => {
                input.incr(some.len());
                Ok(Self(some))
            }
            None => Err(LexerError::ExpectedIdent),
        }
    }
}

pub struct Str<'a>(pub &'a str);

impl<'a> Token<'a> for Str<'a> {
    fn token(input: &mut Lexer<'a>) -> Result<Self, LexerError> {
        let src = input.src()?;

        match super::xmlstr::xmlstr(src) {
            Some(some) => {
                input.incr(some.len());
                Ok(Self(some))
            }
            None => Err(LexerError::ExpectedStr),
        }
    }
}

pub struct Text<'a>(pub &'a str);

impl<'a> Token<'a> for Text<'a> {
    fn token(input: &mut Lexer<'a>) -> Result<Self, LexerError> {
        let src = input.src()?;

        match src.split_once('<') {
            Some((text, _)) => {
                input.incr(text.len());
                Ok(Self(text))
            }
            None => {
                input.incr(src.len());
                Ok(Self(src))
            }
        }
    }
}
