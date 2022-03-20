use crate::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn pipe_ok() {
    let mut tokenizer = Tokenizer::new("|");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_pipe(true).is_ok())
}

#[test]
fn pipe_err() {
    let mut tokenizer = Tokenizer::new("# Comment");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_pipe(true).is_err())
}
