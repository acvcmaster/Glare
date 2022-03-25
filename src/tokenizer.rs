use regex::Regex;

use crate::token::Token;

pub enum TokenizerSpecType {
    Number,
    String,
    SimpleType,
    Pipe,
    Skip,
    Tab,
    Variable,
    Colon,
    Equal,
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
            // Numbers
            TokenizerSpec {
                regex: Regex::new(r"^\d+").unwrap(),
                kind: TokenizerSpecType::Number,
            },
            // String delimiters
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
            // Separators
            TokenizerSpec {
                regex: Regex::new(r"^(?:\t|(?:[ ]{4})){1}").unwrap(),
                kind: TokenizerSpecType::Tab,
            },
            TokenizerSpec {
                regex: Regex::new(r"^\|").unwrap(),
                kind: TokenizerSpecType::Pipe,
            },
            TokenizerSpec {
                regex: Regex::new(r"^\s+").unwrap(),
                kind: TokenizerSpecType::Skip,
            },
            TokenizerSpec {
                regex: Regex::new(r"^\#.*").unwrap(),
                kind: TokenizerSpecType::Skip,
            },
            // Types
            TokenizerSpec {
                regex: Regex::new(r"^(?:Number|str|String|None|\(\)|Never|List|Array)").unwrap(),
                kind: TokenizerSpecType::SimpleType,
            },
            TokenizerSpec {
                regex: Regex::new(r#"^:"#).unwrap(),
                kind: TokenizerSpecType::Colon,
            },
            TokenizerSpec {
                regex: Regex::new(r"^=").unwrap(),
                kind: TokenizerSpecType::Equal,
            },
            // Variable
            TokenizerSpec {
                regex: Regex::new(r"^[^0-9]{1}\w*").unwrap(),
                kind: TokenizerSpecType::Variable,
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

    pub fn get_next_token(&mut self, consume: bool) -> Result<Option<Token>, String> {
        if self.has_more_tokens() {
            for spec in &self.specs {
                if let Some(correspondence) = spec.regex.captures(&self.string[self.cursor..]) {
                    match correspondence.iter().next() {
                        Some(Some(value)) => {
                            let value = value.as_str();
                            let length = value.len();

                            let result = match spec.kind {
                                TokenizerSpecType::Number => match value.parse() {
                                    Ok(parsed) => Ok(Some(Token::Number(parsed))),
                                    Err(_) => {
                                        Err(format!("{} is too large for type 'Number'", value))
                                    }
                                },
                                TokenizerSpecType::String => {
                                    Ok(Some(Token::String((&value[1..length - 1]).to_owned())))
                                }
                                TokenizerSpecType::SimpleType => {
                                    Ok(Some(Token::SimpleType(value.to_owned())))
                                }
                                TokenizerSpecType::Pipe => Ok(Some(Token::Pipe)),
                                TokenizerSpecType::Skip => {
                                    // Skip this token
                                    self.cursor += length;
                                    return self.get_next_token(consume);
                                }
                                TokenizerSpecType::Tab => Ok(Some(Token::Tab)),
                                TokenizerSpecType::Variable => {
                                    Ok(Some(Token::Variable(value.to_owned())))
                                }
                                TokenizerSpecType::Colon => Ok(Some(Token::Colon)),
                                TokenizerSpecType::Equal => Ok(Some(Token::Equal)),
                            };

                            if consume {
                                self.cursor += length;
                            }

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

        /* No more tokens */
        Ok(None)
    }

    fn has_more_tokens(&mut self) -> bool {
        self.cursor < self.string.len()
    }
}
