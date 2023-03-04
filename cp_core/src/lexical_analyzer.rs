use crate::Category;

pub struct Token {
    row: usize,
    column: usize,
    category: Category,
    value: String,
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "row: {}, column: {}, category: {}, value: {}",
//             self.row, self.column, self.category, self.value
//         )
//     }
// }

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

pub fn analyze(text: &String) -> Vec<Token> {
    let lines = text.lines();
    let row = 1;
    for line in lines {
        generate_token(&row, line);
    }
    Vec::new()
}

fn generate_token(row: &usize, line: &str) -> () {
    let bytes = line.as_bytes();
    let mut column = 0;
    while column < line.len() {
        let ch = bytes[column] as char;

        if ch == '_' || ch.is_ascii_alphabetic() {
            // TODO:识别标识符
            ()
        } else if ch == '\'' {
            // TODO:识别字符
            let token = recognize_character(row, &mut column, bytes);
            match token {
                Ok(token) => {
                    println!("{}", token.value);
                }
                Err(token) => {
                    println!("err: {}", token.value);
                }
            }
            ()
        } else if ch == '\"' {
            // TODO:识别字符串
            ()
        } else if ch.is_ascii_digit() {
            // TODO:识别数字
            ()
        } else {
            // TODO:识别其他
            ()
        }

        column += 1;
    }
}

fn recognize_character(row: &usize, column: &mut usize, bytes: &[u8]) -> Result<Token, Token> {
    let mut result = "".to_string();
    let mut status = 0;
    let ch = bytes[*column] as char;
    result.push(ch);

    while status != 2 {
        *column += 1;
        let ch = bytes[*column] as char;
        match status {
            0 => {
                if ch == '\'' {
                    return Err(Token::new(
                        row.clone(),
                        column.clone(),
                        Category::Character,
                        result,
                    ));
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
                    // return set_error(ch, row, column);
                    return Err(Token::new(
                        row.clone(),
                        column.clone(),
                        Category::Character,
                        result,
                    ));
                }
            }
            2 => {}
            3 => {
                if ch == 't' || ch == 'n' || ch == 'r' || ch == '\"' || ch == '\'' {
                    status = 4;
                } else {
                    // return set_error(ch, row, column);
                    return Err(Token::new(
                        row.clone(),
                        column.clone(),
                        Category::Character,
                        result,
                    ));
                }
            }
            4 => {
                if ch == '\'' {
                    status = 2;
                } else {
                    // return set_error(ch, row, column);
                    return Err(Token::new(
                        row.clone(),
                        column.clone(),
                        Category::Character,
                        result,
                    ));
                }
            }
            _ => {}
        }
        result.push(ch);
    }
    Ok(Token::new(
        row.clone(),
        column.clone(),
        Category::Character,
        result,
    ))
}

fn recognize_digit(row: &usize, column: &usize, bytes: &[u8]) {
    let mut result = "".to_string();
    let mut status = 0;
    let mut ch = bytes[*column] as char;
    result.push(ch);
    if ch == '0' {
    } else {
    }
    ()
}
