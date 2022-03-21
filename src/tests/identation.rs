use crate::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn tab_ok() {
    // 3 tabs
    let mut tokenizer = Tokenizer::new("            ");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_identation() {
        Ok(result) => result.count == 3,
        Err(_) => false,
    });
}

#[test]
fn tab_err() {
    let mut tokenizer = Tokenizer::new("Number");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_identation().is_err())
}
