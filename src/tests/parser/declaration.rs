use crate::{
    parser::{Literal, Parser, SimpleType, Variable},
    tests::parser::declaration,
    tokenizer::Tokenizer,
};

#[test]
fn declaration_ok() {
    let declaration = "test = 42";
    let mut tokenizer = Tokenizer::new(declaration);
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_declaration() {
        Ok(declaration) => match (
            declaration.variable,
            declaration.union_type,
            declaration.literal
        ) {
            (Variable { name }, None, Literal::NumberLiteral(literal)) =>
                name == "test" && literal == 42,
            _ => false,
        },
        Err(_) => false,
    })
}

#[test]
fn declaration_type_ok() {
    let declaration = "test: String | str = `Testing declaration`";
    let mut tokenizer = Tokenizer::new(declaration);
    let mut parser = Parser::new(&mut tokenizer);

    assert!(match parser.parse_declaration() {
        Ok(declaration) => match (
            declaration.variable,
            declaration.union_type,
            declaration.literal
        ) {
            (Variable { name }, Some(union_type), Literal::StringLiteral(literal)) =>
                name == "test"
                    && union_type.has_type(SimpleType::String)
                    && union_type.has_type(SimpleType::Str)
                    && literal == "Testing declaration",
            _ => false,
        },
        Err(_) => false,
    })
}

#[test]
fn declaration_err() {
    let mut tokenizer = Tokenizer::new("1234abcd");
    let mut parser = Parser::new(&mut tokenizer);

    assert!(parser.parse_variable().is_err())
}
