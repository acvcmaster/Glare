use crate::{
    parser::{Parser, SimpleType},
    tokenizer::Tokenizer,
};

#[test]
fn union_type_ok() {
    let mut tokenizer = Tokenizer::new("String | str | Number");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_union_type() {
        Ok(parsed) =>
            parsed.has_type(SimpleType::String)
                && parsed.has_type(SimpleType::Str)
                && parsed.has_type(SimpleType::Number),
        Err(_) => false,
    })
}

#[test]
fn union_type_err() {
    let mut tokenizer = Tokenizer::new("String | str | ");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_union_type().is_err())
}
