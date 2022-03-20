use crate::{token::Token, tokenizer::Tokenizer};

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

pub struct UnionType {
    types: Vec<SimpleType>,
}

impl<'a> Parser<'a> {
    /** Initializes the parser. */
    pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    /**
     * Literal
     *  : StringLiteral
     *  | NumberLiteral
     *  ;
     */
    pub fn parse_literal(&mut self, consume: bool) -> Result<Literal, String> {
        match self.tokenizer.get_next_token(consume) {
            Ok(Some(token)) => match token {
                Token::String(string) => Ok(Literal::StringLiteral(string)),
                Token::Number(number) => Ok(Literal::NumberLiteral(number)),
                _ => Err(format!("Expected valid literal value (got {})", token)),
            },
            Ok(None) => Err("Expected valid literal value (got EOF)".to_string()),
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
    pub fn parse_simple_type(&mut self, consume: bool) -> Result<SimpleType, String> {
        match self.tokenizer.get_next_token(consume) {
            Ok(Some(token)) => match token {
                Token::SimpleType(value) => match SimpleType::try_from(value) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error),
                },
                _ => Err(format!("Expected valid simple type (got {})", token)),
            },
            Ok(None) => Err("Expected valid literal value (got EOF)".to_string()),
            Err(error) => Err(error),
        }
    }

    /**
     * Pipe
     *  : |
     * ;
     */
    pub fn parse_pipe(&mut self, consume: bool) -> Result<(), String> {
        match self.tokenizer.get_next_token(consume) {
            Ok(Some(token)) => match token {
                Token::Pipe => Ok(()),
                _ => Err(format!("Expected pipe (got {})", token)),
            },
            Ok(None) => Err("Expected pipe (got EOF)".to_string()),
            Err(error) => Err(error),
        }
    }

    /**
     * UnionType
     *  : SimpleType (Pipe SimpleTipe)*
     * ;
     */
    pub fn parse_union_type(&mut self, consume: bool) -> Result<SimpleType, String> {
        let mut pipe = false;

        loop {
            let error = format!(
                "Expected {} (got EOF)",
                if pipe { "pipe" } else { "simple type" }
            );

            let result = match self.tokenizer.get_next_token(false) {
                Ok(Some(token)) => Ok(()),
                Ok(None) => Err(error),
                Err(error) => Err(error),
            };

            /* Toggle pipe */
            pipe = !pipe;
        }
    }

    /**
     * Type
     *  : UnionType
     *  | SimpleType
     * ;
     */
    pub fn parse_type(&mut self, consume: bool) -> Result<Type, String> {
        match self.parse_simple_type(consume) {
            Ok(value) => Ok(Type::SimpleType(value)),
            Err(error) => Err(error),
        }
    }
}
