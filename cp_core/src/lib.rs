pub mod lexical_analyzer;

#[warn(dead_code)]

enum Category {
    KeyWords,
    Delimiter,
    Litter,
    Character,
}

enum KeyWords {
    CHAR,
    INT,
    FLOAT,
    BREAK,
    CONST,
    RETURN,
    VOID,
    CONTINUE,
    DO,
    WHILE,
    IF,
    ELSE,
    FOR,
}

enum Delimiter {
    LEFTCURLYBRACKET,
    RIGHTCURLYBRACKET,
    SEMICOLON,
    COMMA,
}

enum Litter {
    INTEGER,
    CHARACTER,
    STRING,
    INEDNTIFIER,
    FLOAT,
}

// enum Operator {
//
// }

pub fn count_line(text: &String) -> usize {
    text.split('\n').count()
}
