mod cursor;

pub use crate::token::*;
use crate::error::LexerError;
pub use cursor::Cursor;

type LexerResult = Result<Token, LexerError>;