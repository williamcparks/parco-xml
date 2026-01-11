mod ident;
mod lexer;
mod skip;
mod tokens;
mod traits;
mod xmlstr;

pub use lexer::Lexer;
pub use tokens::{CloseAngle, Colon, Equal, Ident, OpenAngle, Slash, Str, Text};
pub use traits::{LexerError, Token};
