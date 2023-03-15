use crate::token::Numbers::*;
use crate::token::{KeyWords, Numbers, Operators, Token, TokenKind};
use std::str::Chars;
type LexerResult = Result<Token, Token>;
const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    chars: Chars<'a>,
    current: char,
    row: usize,
    column: usize,
}

// #[derive(Debug)]
// enum LexerError {
//     Success,
//     UnexpectEnd,
//     UnexpectedChar,
// }

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            current: EOF_CHAR,
            row: 1,
            column: 1,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    fn next(&mut self) -> Option<char> {
        if self.current == '\n' {
            self.row += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        let output = match self.chars.next() {
            Some(ch) => Some(ch),
            _ => None,
        };
        self.current = self.first();
        output
    }
}

fn is_whitespace(ch: char) -> bool {
    matches!(ch, '\t' | '\n' | ' ' | '\r')
}

fn is_operator(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-'
            | '*'
            | '/'
            | '&'
            | '|'
            | '!'
            | '^'
            | '?'
            | ':'
            | '>'
            | '<'
            | '='
            | '('
            | ')'
            | '['
            | ']'
    )
}

fn is_delimiter(ch: char) -> bool {
    matches!(ch, '{' | '}' | ';' | ',')
}

fn is_id_start(ch: char) -> bool {
    ch == '_' || (ch <= 'z' && ch >= 'a') || (ch <= 'Z' && ch >= 'A')
}

fn is_id_continue(ch: char) -> bool {
    ch == '_' || (ch <= 'z' && ch >= 'a') || (ch <= 'Z' && ch >= 'A') || (ch <= '9' && ch >= '0')
}

fn is_number(ch: char) -> bool {
    ch <= '9' && ch >= '0'
}

fn is_whitespace_or_operator_or_delimiter(ch: char) -> bool {
    is_whitespace(ch) || is_operator(ch) || is_delimiter(ch)
}

// fn is_whitespace_or_delimiter(ch: char) -> bool {
//     is_whitespace(ch) || is_delimiter(ch)
// }

fn is_keyword(buf: &str) -> Option<TokenKind> {
    match buf {
        "char" => Some(TokenKind::KeyWord(KeyWords::Char)),
        "int" => Some(TokenKind::KeyWord(KeyWords::Int)),
        "float" => Some(TokenKind::KeyWord(KeyWords::Float)),
        "break" => Some(TokenKind::KeyWord(KeyWords::Break)),
        "const" => Some(TokenKind::KeyWord(KeyWords::Const)),
        "return" => Some(TokenKind::KeyWord(KeyWords::Return)),
        "void" => Some(TokenKind::KeyWord(KeyWords::Void)),
        "continue" => Some(TokenKind::KeyWord(KeyWords::Continue)),
        "do" => Some(TokenKind::KeyWord(KeyWords::Do)),
        "while" => Some(TokenKind::KeyWord(KeyWords::While)),
        "if" => Some(TokenKind::KeyWord(KeyWords::If)),
        "else" => Some(TokenKind::KeyWord(KeyWords::Else)),
        "for" => Some(TokenKind::KeyWord(KeyWords::For)),
        _ => None,
    }
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> LexerResult {
        match self.first() {
            '/' => match self.second() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => self.operator(),
            },

            ch if is_whitespace(ch) => self.whitespace(),

            ch if is_id_start(ch) => self.ident(),

            ch if is_number(ch) => self.number(),

            '\'' => self.character(),

            '"' => self.str(),

            ch if is_operator(ch) => self.operator(),

            ch if is_delimiter(ch) => self.delimiter(),

            _ => self.unknown(),
        }
    }

    fn line_comment(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                // NOTE:通过外层判断，状态0和状态1应该一定有
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 2;
                }
                2 => match self.first() {
                    // NOTE:去往终结状态，但不取字符
                    '\n' => status = 3,
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                },
                3 => break,
                _ => {}
            }
        }
        // NOTE: line_comment不会遇到错误，读取到文件尾却没到终结状态不属于错误
        // 比如整个文件只有一行注释
        Ok(Token::new(row, column, TokenKind::Comment, buf))
    }

    fn block_comment(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                // NOTE:通过外层判断，状态0和状态1应该一定有
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 2;
                }
                2 => match self.first() {
                    '*' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                },
                3 => match self.first() {
                    // NOTE: 这里与单行注释不同，去往终结状态但取字符
                    '/' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 4;
                    }
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                },
                4 => return Ok(Token::new(row, column, TokenKind::Comment, buf)),
                _ => {}
            }
        }
        // TODO: 设置错误类型
        // NOTE: 当读取到文件尾却没到终结状态时，会产生未闭合错误
        Err(Token::new(row, column, TokenKind::Comment, buf))
    }

    fn whitespace(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => break,
                _ => {}
            }
        }
        // NOTE: 这里可能不会产生错误
        // TODO: 有时间仔细思考一下
        Ok(Token::new(row, column, TokenKind::WhiteSpace, buf))
    }

    fn ident(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                // NOTE: 根据外部判断，0状态直接取
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => match self.first() {
                    ch if is_id_continue(ch) => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                    // NOTE: 这里与单行注释相同，与多行注释不同
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 2,
                    // NOTE: 这里产生错误，需要一直读取到空或界或运
                    // 因为这里与运算符不一样，对于运算符，只读取到空或界
                    // 那属于运算符错误
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // 读到文件尾部
                        // 两种方式相同，可以合并
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                2 => break,
                _ => {}
            }
        }
        // NOTE: 读到文件尾，却没有遇到终结状态，可以有这种情况
        // 因此可以在循环外处理正确返回
        // 判断是否为关键字
        match is_keyword(buf.as_str()) {
            Some(kind) => Ok(Token::new(row, column, kind, buf)),
            None => Ok(Token::new(row, column, TokenKind::Identifier, buf)),
        }
    }

    fn number(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => match self.first() {
                    // NOTE: 由外层判断决定，此处不会有其他情况
                    '1'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                    '0' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                    _ => {}
                },
                1 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 7;
                    }
                    '.' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 8;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                2 => match self.first() {
                    '0' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                    '1'..='7' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    'x' | 'X' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 4;
                    }
                    'b' | 'B' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 5;
                    }
                    '.' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 8;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                3 => match self.first() {
                    '0'..='7' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                4 => match self.first() {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 4;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                5 => match self.first() {
                    '0' | '1' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 5;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                6 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Number(Numbers::Integer),
                        buf,
                    ));
                }
                7 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 7;
                    }
                    '.' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 8;
                    }
                    'e' | 'E' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 11;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 6,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                8 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 9;
                    }
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                9 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 9;
                    }

                    'e' | 'E' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 11;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 10,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                10 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Number(Numbers::Float),
                        buf,
                    ));
                }
                11 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 11;
                    }
                    '+' | '-' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 12;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 14,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                12 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 13;
                    }
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                13 => match self.first() {
                    '0'..='9' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 13;
                    }
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 14,
                    _ => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                },
                14 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Number(Numbers::Exponent),
                        buf,
                    ));
                }
                _ => {}
            }
        }
        // FIX: 如果数字的结尾后没有其他东西，会走到这里，但是无法知道类型
        // TODO: 考虑读到文件尾的情况
        Ok(Token::new(row, column, TokenKind::Number(Integer), buf))
    }

    // FIX：单个引号会被识别为正确
    fn character(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => match self.first() {
                    '\\' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 4;
                    }
                    // TODO: 设置错误类型
                    '\'' => return Err(Token::new(row, column, TokenKind::Character, buf)),
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                },
                2 => match self.first() {
                    '\'' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    // TODO: 这里应该需要特殊考虑
                    _ => return Err(Token::new(row, column, TokenKind::Character, buf)),
                },
                3 => break,
                4 => match self.first() {
                    'n' | 't' | '\\' | 'r' | '\'' | '"' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                    // TODO: 这里应该需要特殊考虑
                    _ => return Err(Token::new(row, column, TokenKind::Character, buf)),
                },
                _ => {}
            }
        }
        // TODO: 考虑读到文件尾的问题
        Ok(Token::new(row, column, TokenKind::Character, buf))
    }
    // FIX: 单个引号会被识别为正确
    fn str(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => match self.first() {
                    '"' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                    '\\' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    '\n' => {
                        while !self.is_eof() {
                            match self.first() {
                                ch if is_whitespace_or_operator_or_delimiter(ch) => break, // 没有读到文件尾
                                _ => {
                                    let ch = self.next().unwrap();
                                    buf.push(ch);
                                }
                            }
                        }
                        // TODO: 设置错误类型
                        return Err(Token::new(row, column, TokenKind::Identifier, buf));
                    }
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                },
                2 => return Ok(Token::new(row, column, TokenKind::String, buf)),
                3 => match self.first() {
                    'n' | 't' | '\\' | 'r' | '\'' | '"' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                    '\n' => {
                        // NOTE: 注意这里没有取换行符
                        self.next();
                        status = 1;
                    }
                    _ => return Err(Token::new(row, column, TokenKind::String, buf)),
                },
                _ => {}
            }
        }
        Err(Token::new(row, column, TokenKind::String, buf))
    }

    fn operator(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => match self.first() {
                    '+' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                    '-' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 2;
                    }
                    '*' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 3;
                    }
                    '/' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 4;
                    }
                    '%' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 5;
                    }
                    '>' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 6;
                    }
                    '<' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 7;
                    }
                    '&' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 8;
                    }
                    '|' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 9;
                    }
                    '!' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 10;
                    }
                    '^' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 11;
                    }
                    '?' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 12;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 13;
                    }
                    '(' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 14;
                    }
                    ')' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 15;
                    }
                    '[' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 16;
                    }
                    ']' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 17;
                    }
                    _ => {}
                },
                1 => match self.first() {
                    '+' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 18;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 19;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 20;
                    }
                },
                2 => match self.first() {
                    '-' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 21;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 22;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 23;
                    }
                },
                3 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 24;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 25;
                    }
                },
                4 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 26;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 27
                    }
                },
                5 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 28;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 29;
                    }
                },
                6 => match self.first() {
                    '>' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 30;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 31;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 32;
                    }
                },
                7 => match self.first() {
                    '<' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 33;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 34;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 35;
                    }
                },
                8 => match self.first() {
                    '&' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 36;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 37;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 38;
                    }
                },
                9 => match self.first() {
                    '|' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 39;
                    }
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 40;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 41;
                    }
                },
                10 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 42;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 43;
                    }
                },
                11 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 44;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 45;
                    }
                },
                12 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Question),
                        buf,
                    ))
                }
                13 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 46;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 47;
                    }
                },
                14 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::OpenParen),
                        buf,
                    ))
                }
                15 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::CloseParen),
                        buf,
                    ))
                }
                16 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::OpenBracket),
                        buf,
                    ))
                }
                17 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::CloseBracket),
                        buf,
                    ))
                }
                18 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleAdd),
                        buf,
                    ))
                }
                19 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::AddEqual),
                        buf,
                    ))
                }
                20 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Add),
                        buf,
                    ))
                }
                21 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleMinus),
                        buf,
                    ))
                }
                22 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::MinusEqual),
                        buf,
                    ))
                }
                23 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Minus),
                        buf,
                    ))
                }
                24 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::MulEqula),
                        buf,
                    ))
                }
                25 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Mul),
                        buf,
                    ))
                }
                26 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DivisionEqual),
                        buf,
                    ))
                }
                27 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Division),
                        buf,
                    ))
                }
                28 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::PercentEqual),
                        buf,
                    ))
                }
                29 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Percent),
                        buf,
                    ))
                }
                30 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 48;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 49;
                    }
                },
                31 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::GreaterEqual),
                        buf,
                    ))
                }
                32 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Greater),
                        buf,
                    ))
                }
                33 => match self.first() {
                    '=' => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 50;
                    }
                    _ => {
                        // let ch = self.next().unwrap();
                        // buf.push(ch);
                        status = 51;
                    }
                },
                34 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::LessEqual),
                        buf,
                    ))
                }
                35 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Less),
                        buf,
                    ))
                }
                36 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::LogicAnd),
                        buf,
                    ))
                }
                37 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::AndEqual),
                        buf,
                    ))
                }
                38 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::And),
                        buf,
                    ))
                }
                39 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::LogicOr),
                        buf,
                    ))
                }
                40 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::OrEqual),
                        buf,
                    ))
                }
                41 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Or),
                        buf,
                    ))
                }
                42 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::NotEqual),
                        buf,
                    ))
                }
                43 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Not),
                        buf,
                    ))
                }
                44 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::XorEqual),
                        buf,
                    ))
                }
                45 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Xor),
                        buf,
                    ))
                }
                46 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::LogicEqual),
                        buf,
                    ))
                }
                47 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::Equal),
                        buf,
                    ))
                }
                48 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleGreaterEqual),
                        buf,
                    ))
                }
                49 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleGreater),
                        buf,
                    ))
                }
                50 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleLessEqual),
                        buf,
                    ))
                }
                51 => {
                    return Ok(Token::new(
                        row,
                        column,
                        TokenKind::Operator(Operators::DoubleLess),
                        buf,
                    ))
                }
                _ => {}
            }
        }
        // TODO: 处理读取到文件尾的情况
        Ok(Token::new(row, column, TokenKind::TODO, buf))
    }

    fn delimiter(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => break,
                _ => {}
            }
        }
        Ok(Token::new(row, column, TokenKind::Delimiter, buf))
    }

    fn unknown(&mut self) -> LexerResult {
        let row = self.row.clone();
        let column = self.column.clone();
        let mut buf = String::from("");

        let mut status = 0;
        while !self.is_eof() {
            match status {
                0 => {
                    let ch = self.next().unwrap();
                    buf.push(ch);
                    status = 1;
                }
                1 => match self.first() {
                    ch if is_whitespace_or_operator_or_delimiter(ch) => status = 2,
                    _ => {
                        let ch = self.next().unwrap();
                        buf.push(ch);
                        status = 1;
                    }
                },
                2 => break,
                _ => break,
            }
        }
        Err(Token::new(row, column, TokenKind::Unknown, buf))
    }
}
