use crate::Category;

pub struct Token {
    row: usize,
    column: usize,
    category: Category,
    value: String,
}

impl Token {
    fn new(row: usize, column: usize, category: Category, value: String) -> Self {
        Self {
            row,
            column,
            category,
            value,
        }
    }
}

pub fn analyze(source_code: &str) -> Vec<Token> {
    generate_tokens(source_code);
    Vec::new()
}

fn generate_tokens(source_code: &str) -> (Vec<Token>, Vec<Token>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut logs: Vec<Token> = Vec::new();
    for (row, line) in source_code.lines().enumerate() {
        let mut column = 0;

        while column < line.len() {
            let ch = line.chars().nth(column).unwrap();
            if ch == '_' || ch.is_ascii_alphabetic() {
                // TODO:识别标识符
                let (end, token) = recognize_identifier(&row, &column, &line[column..]);
                column += end + 1;

                match token {
                    Some(token) => match token {
                        Ok(token) => {
                            println!("{}", token.value);
                            tokens.push(token);
                        }
                        Err(token) => {
                            println!("err {}", token.value);
                            logs.push(token);
                        }
                    },
                    None => {
                        println!("none!!!!");
                    }
                }
            } else if ch == '\'' {
                // TODO:识别字符
                let (end, token) = recognize_character(&row, &column, &line[column + 1..]);
                column += end + 1;
                match token {
                    Some(token) => match token {
                        Ok(token) => {
                            println!("{}", token.value);
                            tokens.push(token);
                        }
                        Err(token) => {
                            println!("err {}", token.value);
                            logs.push(token);
                        }
                    },
                    None => {
                        println!("none!!!!");
                    }
                }
            } else if ch == '\"' {
                // TODO:识别字符串
            } else if ch.is_ascii_digit() {
                // TODO:识别数字
            } else {
                // TODO:识别其他
            }
            column += 1;
        }
    }
    (tokens, logs)
}

fn set_ok(row: usize, column: usize, value: String) -> Result<Token, Token> {
    Ok(Token::new(
        row.clone(),
        column.clone(),
        Category::Character,
        value,
    ))
}

fn set_err(row: usize, column: usize, value: String) -> Result<Token, Token> {
    Err(Token::new(
        row.clone(),
        column.clone(),
        Category::Character,
        value,
    ))
}

fn recognize_character(
    row: &usize,
    column: &usize,
    line: &str,
) -> (usize, Option<Result<Token, Token>>) {
    let mut end = 0;
    let mut status = 0;
    let mut value = String::from("\'");

    for (index, ch) in line.chars().enumerate() {
        value.push(ch);
        match status {
            0 => {
                if ch == '\'' {
                    // err
                    break;
                } else if ch == '\\' {
                    status = 3;
                } else {
                    status = 1;
                }
            }
            1 => {
                if ch == '\'' {
                    status = 2;
                } else {
                    // err
                    break;
                }
            }
            2 => {
                break;
            }
            3 => {
                if ch == 't' || ch == 'n' || ch == 'r' || ch == '\"' || ch == '\'' {
                    status = 4;
                } else {
                    // err
                    break;
                }
            }
            4 => {
                if ch == '\'' {
                    status = 2;
                } else {
                    // err
                    break;
                }
            }
            _ => {}
        }
        end = index;
    }

    if status == 2 {
        (
            end,
            Some(set_ok(row.clone(), column.clone(), value.clone())),
        )
    } else {
        (
            end,
            Some(set_err(row.clone(), column.clone(), value.clone())),
        )
    }
}

fn recognize_identifier(
    row: &usize,
    column: &usize,
    line: &str,
) -> (usize, Option<Result<Token, Token>>) {
    let mut end = 0;
    let mut status = 0;
    let mut value = String::from("");

    for (index, ch) in line.chars().enumerate() {
        value.push(ch);
        match status {
            0 => {
                if ch == '_' || ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                    status = 1;
                } else {
                    // err
                    break;
                }
            }
            1 => {
                if ch == '_' || ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                    status = 1;
                } else if is_space_or_operator(ch) {
                    status = 2;
                } else {
                    // err
                    break;
                }
            }
            2 => {
                break;
            }
            _ => {}
        }
        end = index;
    }
    if status == 2 || end >= line.len() {
        (
            end,
            Some(set_ok(row.clone(), column.clone(), value.clone())),
        )
    } else {
        (
            end,
            Some(set_err(row.clone(), column.clone(), value.clone())),
        )
    }
}

fn is_space_or_operator(ch: char) -> bool {
    ch == ' '
        || ch == '+'
        || ch == '-'
        || ch == '*'
        || ch == '/'
        || ch == '%'
        || ch == '&'
        || ch == '|'
        || ch == '!'
        || ch == '<'
        || ch == '>'
        || ch == '='
        || ch == '?'
}

// fn recognize_digit(
//     row: &usize,
//     column: &usize,
//     line: &str,
// ) -> (usize, Option<Result<Token, Token>>) {
//     let mut end = 0;
//     let mut status = 0;
//     let mut value = String::from("");
//
//     for (index, ch) in line.chars().enumerate() {
//         // if index == 0 && ch == '0' {
//         // } else {
//         // }
//         match status {
//             0 => {
//                 if ch == '0' {
//                     status = 1;
//                 } else {
//                     status = 7;
//                 }
//             }
//             1 => {
//                 if ch == '0' {
//                     status = 2;
//                 } else if {
//
//                 }
//             }
//             2 => {}
//             3 => {}
//             4 => {}
//             5 => {}
//             6 => {}
//             7 => {}
//             8 => {}
//             9 => {}
//         }
//     }
// }
