use regex::Regex;

use crate::token::Token;

pub enum TokenizerSpecType {
    Number,
    String,
    Whitespace,
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
                kind: TokenizerSpecType::Whitespace,
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
                            let length = value.as_str().len();

                            let result = match spec.kind {
                                TokenizerSpecType::Number => match value.as_str().parse() {
                                    Ok(parsed) => Ok(Token::Number(parsed)),
                                    Err(_) => continue,
                                },
                                TokenizerSpecType::String => {
                                    Ok(Token::String((&value.as_str()[1..length - 1]).to_owned()))
                                }
                                _ => {
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
