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

impl PartialEq for SimpleType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
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

#[derive(Debug)]
pub struct UnionType {
    pub types: Vec<SimpleType>,
}

impl UnionType {
    pub fn has_type(&self, simple_type: SimpleType) -> bool {
        self.types.contains(&simple_type)
    }
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
    pub fn parse_literal(&mut self) -> Result<Literal, String> {
        match self.tokenizer.get_next_token(true) {
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
    pub fn parse_simple_type(&mut self) -> Result<SimpleType, String> {
        match self.tokenizer.get_next_token(true) {
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
    pub fn parse_pipe(&mut self) -> Result<(), String> {
        match self.tokenizer.get_next_token(true) {
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
     *  : SimpleType
     *  : SimpleType Pipe UnionType
     * ;
     */
    pub fn parse_union_type(&mut self) -> Result<UnionType, String> {
        match self.tokenizer.get_next_token(true) {
            Ok(Some(token)) => match &token {
                Token::SimpleType(simple_type) => {
                    match SimpleType::try_from(simple_type.to_owned()) {
                        Ok(simple_type) => {
                            let mut types = vec![simple_type];

                            if let Ok(Some(Token::Pipe)) = self.tokenizer.get_next_token(false) {
                                if self.tokenizer.get_next_token(true).is_ok() {
                                    let next = self.parse_union_type();
                                    return match next {
                                        Ok(next) => {
                                            for simple_type in next.types {
                                                types.push(simple_type);
                                            }
                                            return Ok(UnionType { types });
                                        }
                                        Err(error) => Err(error),
                                    };
                                }
                            }
                            
                            Ok(UnionType { types })
                        }
                        Err(_) => Err(format!("Expected valid simple type (got {})", token)),
                    }
                }
                _ => Err(format!("Expected simple type (got {})", token)),
            },
            Ok(None) => Err("Expected simple type (got EOF)".to_string()),
            Err(error) => Err(error),
        }
    }
}
