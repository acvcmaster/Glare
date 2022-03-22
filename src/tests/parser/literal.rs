use crate::{
    parser::{Literal, Parser},
    tokenizer::Tokenizer,
};

#[test]
fn string_literal_ok() {
    let mut tokenizer = Tokenizer::new("`Literal string`");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_literal() {
        Ok(value) => match value {
            Literal::StringLiteral(value) => value == "Literal string",
            Literal::NumberLiteral(_) => false,
        },
        _ => false,
    })
}

#[test]
fn number_literal_ok() {
    let mut tokenizer = Tokenizer::new("42");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_literal() {
        Ok(value) => match value {
            Literal::StringLiteral(_) => false,
            Literal::NumberLiteral(value) => value == 42,
        },
        _ => false,
    })
}

#[test]
fn literal_err() {
    let mut tokenizer = Tokenizer::new("# Comment");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_literal().is_err())
}
