use std::fmt::Display;

use logos::Logos;

#[derive(Debug)]
pub struct Token {
    row: usize,
    column: usize,
    token_kind: TokenKind,
    value: String,
}

impl Token {
    fn new(row: usize, column: usize, token_kind: TokenKind, value: String) -> Self {
        Self {
            row,
            column,
            token_kind,
            value,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}, value: {}, token_kind: {:?}",
            self.row, self.column, self.value, self.token_kind
        )
    }
}

pub struct ReLexer<'a> {
    source_code: &'a str,
    pub tokens: Vec<Token>,
    pub errors: Vec<Token>,
}

impl<'a> ReLexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            tokens: Vec::<Token>::new(),
            errors: Vec::<Token>::new(),
        }
    }

    pub fn generate_token(&mut self) {
        let mut row = 1;
        let mut column = 1;
        let mut lex = TokenKind::lexer(self.source_code);
        while let Some(token) = lex.next() {
            match token {
                TokenKind::Comment => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Comment,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Delimiter => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Delimiter,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Integer => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Integer,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Float => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Float,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Exponent => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Exponent,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Identifier => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Identifier,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Character => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Character,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::String => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::String,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Add => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Add,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Minus => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Minus,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Multiply => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Multiply,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Divison => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Divison,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Percent => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Percent,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::DoubleEqual => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::DoubleEqual,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::NotEqual => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::NotEqual,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Greater => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Greater,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Less => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Less,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::GreaterEqual => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::GreaterEqual,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::LessEqual => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::LessEqual,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::LogicNot => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::LogicNot,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::LogicAnd => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::LogicAnd,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::LogicOr => {
                    self.tokens.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::LogicOr,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::Error => {
                    self.errors.push(Token::new(
                        row.clone(),
                        column.clone(),
                        TokenKind::Error,
                        lex.slice().to_string(),
                    ));
                    column += lex.span().end - lex.span().start;
                }
                TokenKind::NewLine => {
                    row += 1;
                    column = 1;
                }
                TokenKind::WhiteSpace => {
                    column += lex.span().end - lex.span().start;
                }
            }
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
enum TokenKind {
    #[regex(r#"(/\*[^*]*\*+([^/*][^*]*\*+)*/)|(//[^\n]*)"#)]
    Comment,

    #[regex(r#" |\t"#)]
    WhiteSpace,

    #[regex(r#",|;|\{|\}"#)]
    Delimiter,

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
    Identifier,

    #[regex(r#"0|[1-9]\d*"#)]
    Integer,

    #[regex(r#"[1-9]\d*\.\d*|0\.\d*[1-9]\d*"#)]
    Float,

    #[regex(r#"[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)"#)]
    Exponent,

    #[regex(r#"'[^']+'"#)]
    Character,

    #[regex(r#""[^"]*""#)]
    String,

    #[regex(r#"\+"#)]
    Add,

    #[regex(r#"-"#)]
    Minus,

    #[regex(r#"\*"#)]
    Multiply,

    #[regex(r#"/"#)]
    Divison,

    #[regex(r#"%"#)]
    Percent,

    #[regex(r#"=="#)]
    DoubleEqual,

    #[regex(r#"!="#)]
    NotEqual,

    #[regex(r#">"#)]
    Greater,

    #[regex(r#"<"#)]
    Less,

    #[regex(r#">="#)]
    GreaterEqual,

    #[regex(r#"<="#)]
    LessEqual,

    #[regex(r#"!"#)]
    LogicNot,

    #[regex(r#"&&"#)]
    LogicAnd,

    #[regex(r#"\|\|"#)]
    LogicOr,

    #[regex(r#"\r|\n|\r\n"#)]
    NewLine,

    #[error]
    Error,
}
