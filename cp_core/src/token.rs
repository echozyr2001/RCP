use std::fmt::Display;

#[derive(Debug)]
pub struct Token {
    row: usize,
    column: usize,
    token_kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(row: usize, column: usize, token_kind: TokenKind, value: String) -> Self {
        Self {
            row,
            column,
            token_kind,
            value,
        }
    }

    pub fn not_whitespace(&self) -> bool {
        self.token_kind != TokenKind::WhiteSpace
    }

    pub fn not_comment(&self) -> bool {
        self.token_kind != TokenKind::Comment
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}, \t\tvalue: {}, \t\token_kind: {:?}",
            self.row, self.column, self.value, self.token_kind,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Comment,
    WhiteSpace,
    Delimiter,
    Identifier,
    KeyWord(KeyWords),
    Number(Numbers),
    Character,
    String,
    Operator(Operators),
    TODO,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Numbers {
    Integer,
    Exponent,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Division,
    DivisionEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    LogicAnd,
    LogicEqual,
    LogicOr,
    Minus,
    MinusEqual,
    Mul,
    MulEqula,
    Not,
    NotEqual,
    Or,
    OrEqual,
    Percent,
    PercentEqual,
    Question,
    Xor,
    XorEqual,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
}
