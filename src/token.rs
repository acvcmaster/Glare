use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    String(String),
    Number(i64),
    SimpleType(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::String(value) => write!(f, "string '{}'", value),
            Token::Number(value) => write!(f, "number '{}'", value),
            Token::SimpleType(value) => write!(f, "type '{}'", value),
        }
    }
}
