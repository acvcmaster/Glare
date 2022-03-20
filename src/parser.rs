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
    pub fn literal(&mut self) -> Result<Literal, String> {
        match self.tokenizer.get_next_token() {
            Ok(token) => match token {
                Token::String(string) => Ok(Literal::StringLiteral(string)),
                Token::Number(number) => Ok(Literal::NumberLiteral(number)),
                _ => Err(get_translated(TranslationKey::ExpectedLiteral).to_string()),
            },
            Err(error) => Err(error),
        }
    }
}
