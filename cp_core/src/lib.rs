pub mod lexical_analyzer;

#[warn(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    Comment,
    WhiteSpace,
    Delimiter,
    Identifier,
    KeyWord(KeyWords),
    Number(Numbers),
    Character,
    String,
    Operator(Operators),
    Err,
    TODO,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum KeyWords {
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
enum Numbers {
    Integer,
    Exponent,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operators {
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
    LogicNot,
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
