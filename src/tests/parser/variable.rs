use crate::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn variable_ok() {
    let variable_name = "abcd1234";
    let mut tokenizer = Tokenizer::new(variable_name);
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_variable() {
        Ok(variable) => variable.name == variable_name,
        Err(_) => false,
    })
}

#[test]
fn variabe_err() {
    let mut tokenizer = Tokenizer::new("1234abcd");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_variable().is_err())
}
