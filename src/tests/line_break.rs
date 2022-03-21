use crate::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn line_break_ok() {
    let mut tokenizer = Tokenizer::new("\r\n");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_line_break().is_ok())
}

#[test]
fn pipe_err() {
    let mut tokenizer = Tokenizer::new("Number");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_line_break().is_err())
}
