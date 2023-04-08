pub struct LexerError {
  pub message: String,
  pub row: usize,
  pub col: usize,
}

impl LexerError {
  pub fn new(message: String, row: usize, col: usize) -> Self {
    Self {
      message,
      row,
      col,
    }
  }
}

struct ParserError {
  pub message: String,
  pub line: usize,
  pub column: usize,
}