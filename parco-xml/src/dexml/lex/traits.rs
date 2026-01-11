use super::Lexer;

use thiserror::Error;

pub trait Token<'a>: Sized {
    fn token(input: &mut Lexer<'a>) -> Result<Self, LexerError>;
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Failed To Slice XML Source @ `{0}`")]
    Slice(usize),

    #[error("Expected '<'")]
    ExpectedOpenAngle,

    #[error("Expected '>'")]
    ExpectedCloseAngle,

    #[error("Expected '='")]
    ExpectedEqual,

    #[error("Expected '/'")]
    ExpectedSlash,

    #[error("Expected '/'")]
    ExpectedColon,

    #[error("Expected Identifier '[A-Za-z_][A-Za-z0-9_.-]*'")]
    ExpectedIdent,

    #[error("Expected String \"...\"")]
    ExpectedStr,
}
