use crate::{
    token::Token,
    tokenizer::Tokenizer,
    translation::{get_translated, TranslationKey},
};

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
}

#[derive(Debug)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(i64),
}

#[derive(Debug)]
pub enum SimpleType {
    Number,
    Str,
    String,
    None,
    Empty,
    Never,
    List,
}

#[derive(Debug)]
pub enum Type {
    SimpleType(SimpleType),
}

impl TryFrom<String> for SimpleType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Number" => Ok(SimpleType::Number),
            "str" => Ok(SimpleType::Str),
            "String" => Ok(SimpleType::String),
            "None" => Ok(SimpleType::None),
            "()" => Ok(SimpleType::Empty),
            "Never" => Ok(SimpleType::Never),
            "List" => Ok(SimpleType::List),
            _ => Err(format!("The type {} is not valid", value)),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    /**
     * Literal
     *  : StringLiteral
     *  | NumberLiteral
     *  ;
     */
    pub fn parse_literal(&mut self) -> Result<Literal, String> {
        match self.tokenizer.get_next_token() {
            Ok(token) => match token {
                Token::String(string) => Ok(Literal::StringLiteral(string)),
                Token::Number(number) => Ok(Literal::NumberLiteral(number)),
                _ => Err(format!("Expected valid literal value (got {})", token)),
            },
            Err(error) => Err(error),
        }
    }

    /**
     * SimpleType
     *  : Number
     *  | str
     *  | String
     *  | None
     *  | ()
     *  | Never
     *  | List
     * ;
     */
    pub fn parse_simple_type(&mut self) -> Result<SimpleType, String> {
        match self.tokenizer.get_next_token() {
            Ok(token) => match token {
                Token::SimpleType(value) => match SimpleType::try_from(value) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error),
                },
                _ => Err(format!("Expected valid simple type (got {})", token)),
            },
            Err(error) => Err(error),
        }
    }

    /**
     * Type
     *  : SimpleType
     * ;
     */
    pub fn parse_type(&mut self) -> Result<Type, String> {
        match self.parse_simple_type() {
            Ok(value) => Ok(Type::SimpleType(value)),
            Err(error) => Err(error),
        }
    }
}
