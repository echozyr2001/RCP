use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Token {
  token_type: TokenType,
  value: String,
  pos: Option<Point>,
}

impl Token {
  pub fn new(token_type: TokenType, value: String, pos: Option<Point>) -> Self {
    Self {
      token_type,
      value,
      pos,
    }
  }

  pub fn new_terminal(value: String, pos: Option<Point>) -> Self {
    Self::new(TokenType::Terminal(TokenKind::Identifier), value, pos)
  }

  pub fn new_not_terminal(value: String, pos: Option<Point>) -> Self {
    Self::new(TokenType::NotTerminal, value, pos)
  }

  pub fn is_terminal(&self) -> bool {
    if let TokenType::Terminal(_) = self.token_type {
      return true;
    }
    false
  }

  pub fn is_not_terminal(&self) -> bool {
    if let TokenType::NotTerminal = self.token_type {
      return true;
    }
    false
  }

  pub fn get_pos(&self) -> &Option<Point> {
    &self.pos
  }

  pub fn get_value(&self) -> &String {
    &self.value
  }

  pub fn get_type(&self) -> &TokenType {
    &self.token_type
  }

  pub fn get_kind(&self) -> &TokenKind {
    if let TokenType::Terminal(kind) = &self.token_type {
      return kind;
    }
    panic!("Token is not terminal!");
  }

  pub fn is_whitespace(&self) -> bool {
    if let TokenType::Terminal(TokenKind::WhiteSpace) = self.token_type {
      return true;
    }
    false
  }

  pub fn is_comment(&self) -> bool {
    if let TokenType::Terminal(TokenKind::Comment) = self.token_type {
      return true;
    }
    false
  }
}

impl Display for Token {
  // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  //   write!(
  //     f,
  //     "{}@#{}@#{}@#{:?}",
  //     self.pos.unwrap().row, self.pos.unwrap().col, self.value, self.token_type,
  //   )
  // }
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}, row: {}, col: {}",
      self.value, self.pos.unwrap().get_row(), self.pos.unwrap().get_col(),
    )
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub enum TokenType {
  Terminal(TokenKind),
  NotTerminal,
}

impl Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::Terminal(kind) => write!(f, "{}", kind),
      TokenType::NotTerminal => write!(f, "NotTerminal"),
    }
  }
}

impl TokenType {
  pub fn from(s: &str) -> Self {
    match s {
      "err" => TokenType::Terminal(TokenKind::Err),
      "Ident" => TokenType::Terminal(TokenKind::Identifier),
      "IntConst" => TokenType::Terminal(TokenKind::Number(Numbers::Integer)),
      "FloatConst" => TokenType::Terminal(TokenKind::Number(Numbers::Float)),
      "const" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Const)),
      "int" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Int)),
      "float" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Float)),
      "void" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Void)),
      "if" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::If)),
      "else" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Else)),
      "while" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::While)),
      "break" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Break)),
      "continue" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Continue)),
      "return" => TokenType::Terminal(TokenKind::KeyWord(KeyWords::Return)),
      "[" => TokenType::Terminal(TokenKind::Operator(Operators::OpenBracket)),
      "]" => TokenType::Terminal(TokenKind::Operator(Operators::CloseBracket)),
      "=" => TokenType::Terminal(TokenKind::Operator(Operators::Eq)),
      "{" => TokenType::Terminal(TokenKind::OpenBrace),
      "}" => TokenType::Terminal(TokenKind::CloseBrace),
      "," => TokenType::Terminal(TokenKind::Comma),
      ";" => TokenType::Terminal(TokenKind::Semicolon),
      "(" => TokenType::Terminal(TokenKind::OpenParen),
      ")" => TokenType::Terminal(TokenKind::CloseParen),
      "+" => TokenType::Terminal(TokenKind::Operator(Operators::Add)),
      "-" => TokenType::Terminal(TokenKind::Operator(Operators::Sub)),
      "*" => TokenType::Terminal(TokenKind::Operator(Operators::Mul)),
      "/" => TokenType::Terminal(TokenKind::Operator(Operators::Div)),
      "%" => TokenType::Terminal(TokenKind::Operator(Operators::Mod)),
      "!" => TokenType::Terminal(TokenKind::Operator(Operators::Not)),
      "<" => TokenType::Terminal(TokenKind::Operator(Operators::Les)),
      ">" => TokenType::Terminal(TokenKind::Operator(Operators::Grt)),
      "<=" => TokenType::Terminal(TokenKind::Operator(Operators::LesEq)),
      ">=" => TokenType::Terminal(TokenKind::Operator(Operators::GrtEq)),
      "==" => TokenType::Terminal(TokenKind::Operator(Operators::LEq)),
      "&&" => TokenType::Terminal(TokenKind::Operator(Operators::LAnd)),
      "||" => TokenType::Terminal(TokenKind::Operator(Operators::LOr)),
      "ε" => TokenType::Terminal(TokenKind::Empty),
      "#" => TokenType::Terminal(TokenKind::End),
      _ => TokenType::NotTerminal,
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Point {
  row: usize,
  col: usize,
}

impl Point {
  pub fn new(row: usize, col: usize) -> Self {
    Self {
      row,
      col,
    }
  }

  pub fn get_row(&self) -> &usize {
    &self.row
  }

  pub fn get_col(&self) -> &usize {
    &self.col
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub enum TokenKind {
  Comment,
  WhiteSpace,
  // Delimiter,
  Identifier,
  KeyWord(KeyWords),
  Number(Numbers),
  Character,
  String,
  Operator(Operators),
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  Semicolon,
  Comma,
  TODO,
  Empty,
  Unknown,
  Err,
  // TODO:设置start
  Start,
  End,
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenKind::Err => write!(f, "Err"),
      TokenKind::Identifier => write!(f, "Ident"),
      TokenKind::Number(Numbers::Integer) => write!(f, "IntConst"),
      TokenKind::Number(Numbers::Float) => write!(f, "FloatConst"),
      TokenKind::KeyWord(KeyWords::Const) => write!(f, "const"),
      TokenKind::KeyWord(KeyWords::Int) => write!(f, "int"),
      TokenKind::KeyWord(KeyWords::Float) => write!(f, "float"),
      TokenKind::KeyWord(KeyWords::Void) => write!(f, "void"),
      TokenKind::KeyWord(KeyWords::If) => write!(f, "if"),
      TokenKind::KeyWord(KeyWords::Else) => write!(f, "else"),
      TokenKind::KeyWord(KeyWords::While) => write!(f, "while"),
      TokenKind::KeyWord(KeyWords::Break) => write!(f, "break"),
      TokenKind::KeyWord(KeyWords::Continue) => write!(f, "continue"),
      TokenKind::KeyWord(KeyWords::Return) => write!(f, "return"),
      TokenKind::OpenParen => write!(f, "("),
      TokenKind::CloseParen => write!(f, ")"),
      TokenKind::OpenBrace => write!(f, "{{"),
      TokenKind::CloseBrace => write!(f, "}}"),
      TokenKind::Semicolon => write!(f, ";"),
      TokenKind::Comma => write!(f, ","),
      TokenKind::Operator(Operators::OpenBracket) => write!(f, "["),
      TokenKind::Operator(Operators::CloseBracket) => write!(f, "]"),
      // TokenKind::Delimiter => write!(f, ","),
      TokenKind::Operator(Operators::Add) => write!(f, "+"),
      TokenKind::Operator(Operators::Sub) => write!(f, "-"),
      TokenKind::Operator(Operators::Mul) => write!(f, "*"),
      TokenKind::Operator(Operators::Div) => write!(f, "/"),
      TokenKind::Operator(Operators::Mod) => write!(f, "%"),
      TokenKind::Operator(Operators::Not) => write!(f, "!"),
      TokenKind::Operator(Operators::Les) => write!(f, "<"),
      TokenKind::Operator(Operators::Grt) => write!(f, ">"),
      TokenKind::Operator(Operators::LesEq) => write!(f, "<="),
      TokenKind::Operator(Operators::GrtEq) => write!(f, ">="),
      TokenKind::Operator(Operators::LEq) => write!(f, "=="),
      TokenKind::Operator(Operators::NotEq) => write!(f, "!="),
      TokenKind::Operator(Operators::LAnd) => write!(f, "&&"),
      TokenKind::Operator(Operators::LOr) => write!(f, "||"),
      TokenKind::Operator(Operators::Eq) => write!(f, "="),
      TokenKind::Empty => write!(f, "ε"),
      TokenKind::End => write!(f, "#"),
      _ => write!(f, "Unknown"),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub enum KeyWords {
  Char,
  Int,
  Float,
  Break,
  Const,
  Return,
  Void,
  Continue,
  Do,
  While,
  If,
  Else,
  For,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Numbers {
  Integer,
  Exponent,
  Float,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Operators {
  Add,
  AddEqual,
  And,
  AndEqual,
  DoubleAdd,
  DoubleGreater,
  DoubleGreaterEqual,
  DoubleLess,
  DoubleLessEqual,
  DoubleMinus,
  Div,
  DivisionEqual,
  Eq,
  Grt,
  GrtEq,
  Les,
  LesEq,
  LAnd,
  LEq,
  LOr,
  Sub,
  MinusEqual,
  Mul,
  MulEqula,
  Not,
  NotEq,
  Or,
  OrEqual,
  Mod,
  PercentEqual,
  Question,
  Xor,
  XorEqual,
  // OpenParen,
  // CloseParen,
  OpenBracket,
  CloseBracket,
}
