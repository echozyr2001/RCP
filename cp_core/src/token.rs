#[derive(Clone)]
pub enum KeyWords {
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Float,
    For,
    If,
    Int,
    Long,
    Return,
    Short,
    Static,
    Struct,
    Switch,
    Typedef,
    Void,
    While,
}

#[derive(Clone)]
pub enum Operators {
    Add,
    AddEqual,
    And,
    DoubleAdd,
    DoubleMinus,
    Division,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    LogicAnd,
    LogicNot,
    LogicOr,
    Minus,
    MinusEqual,
    Mul,
    Not,
    NotEqual,
    Or,
    Xor,
}

#[derive(Clone)]
pub enum Brackets {}

#[derive(Clone)]
pub enum Numbers {
    Int,
    Float,
}
