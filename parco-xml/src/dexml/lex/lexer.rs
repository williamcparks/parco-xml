use super::{LexerError, Token};

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    xml: &'a str,
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub const fn new(xml: &'a str) -> Self {
        Self { xml, idx: 0 }
    }

    pub const fn incr(&mut self, len: usize) {
        self.idx += len;
    }

    pub const fn xml(&self) -> &'a str {
        self.xml
    }

    pub fn src(&mut self) -> Result<&'a str, LexerError> {
        loop {
            let src = match self.xml.get(self.idx..) {
                Some(some) => some,
                None => return Err(LexerError::Slice(self.idx)),
            };

            match super::skip::skip(src) {
                0 => return Ok(src),
                skip => self.idx += skip,
            }
        }
    }

    pub fn peek(&mut self, ch: char) -> bool {
        let src = match self.src() {
            Ok(ok) => ok,
            _ => return false,
        };
        match src.chars().next() {
            Some(some) => some == ch,
            None => false,
        }
    }

    pub fn token<T: Token<'a>>(&mut self) -> Result<T, LexerError> {
        T::token(self)
    }
}
