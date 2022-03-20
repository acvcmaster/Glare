use regex::Regex;

use crate::token::Token;

pub enum TokenizerSpecType {
    Number,
    String,
    SimpleType,
    Pipe,
    Skip,
}

pub struct TokenizerSpec {
    regex: Regex,
    kind: TokenizerSpecType,
}

pub struct Tokenizer<'a> {
    cursor: usize,
    string: &'a str,
    specs: Vec<TokenizerSpec>,
}

impl<'a> Tokenizer<'a> {
    /** Initializes tokenizer */
    pub fn new(string: &'a str) -> Self {
        let specs = vec![
            TokenizerSpec {
                regex: Regex::new(r"^\d+").unwrap(),
                kind: TokenizerSpecType::Number,
            },
            TokenizerSpec {
                regex: Regex::new(r"^'[^']*'").unwrap(),
                kind: TokenizerSpecType::String,
            },
            TokenizerSpec {
                regex: Regex::new(r#"^"[^"]*""#).unwrap(),
                kind: TokenizerSpecType::String,
            },
            TokenizerSpec {
                regex: Regex::new(r"^`[^`]*`").unwrap(),
                kind: TokenizerSpecType::String,
            },
            TokenizerSpec {
                regex: Regex::new(r"^\s+").unwrap(),
                kind: TokenizerSpecType::Skip,
            },
            TokenizerSpec {
                regex: Regex::new(r"^//.*").unwrap(),
                kind: TokenizerSpecType::Skip,
            },
            TokenizerSpec {
                regex: Regex::new(r"^/\*[\s\S]*?\*/").unwrap(),
                kind: TokenizerSpecType::Skip,
            },
            TokenizerSpec {
                regex: Regex::new(r"^Number|str|String|None|\(\)|Never|List|Array").unwrap(),
                kind: TokenizerSpecType::SimpleType,
            },
            TokenizerSpec {
                regex: Regex::new(r"^\|").unwrap(),
                kind: TokenizerSpecType::Pipe,
            },
        ];

        /* Initializes new tokenizer with the
        cursor set to the initial position */
        Tokenizer {
            string,
            cursor: 0,
            specs,
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        if self.has_more_tokens() {
            for spec in &self.specs {
                if let Some(correspondence) = spec.regex.captures(&self.string[self.cursor..]) {
                    match correspondence.iter().next() {
                        Some(Some(value)) => {
                            let value = value.as_str();
                            let length = value.len();

                            let result = match spec.kind {
                                TokenizerSpecType::Number => match value.parse() {
                                    Ok(parsed) => Ok(Token::Number(parsed)),
                                    Err(_) => {
                                        Err(format!("{} is too large for type 'Number'", value))
                                    }
                                },
                                TokenizerSpecType::String => {
                                    Ok(Token::String((&value[1..length - 1]).to_owned()))
                                }
                                TokenizerSpecType::SimpleType => {
                                    Ok(Token::SimpleType(value.to_owned()))
                                }
                                TokenizerSpecType::Pipe => Ok(Token::Pipe),
                                TokenizerSpecType::Skip => {
                                    // Skip this token
                                    self.cursor += length;
                                    return self.get_next_token();
                                }
                            };

                            self.cursor += length;

                            return result;
                        }
                        _ => continue,
                    }
                }
            }

            if let Some(current_char) = self.string.chars().nth(self.cursor) {
                return Err(format!(
                    "Unexpected symbol {} (at index {})",
                    current_char, self.cursor
                ));
            }

            return Err(format!("Unexpected symbol at index {}", self.cursor));
        }

        Err("Unexpected EOF".to_string())
    }

    fn has_more_tokens(&mut self) -> bool {
        self.cursor < self.string.len()
    }
}
