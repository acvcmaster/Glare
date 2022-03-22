use crate::{
    codegen::{Compilable, Target},
    parser::Literal,
};

#[test]
fn literal_string() {
    let literal = "String Literal";
    let string_literal = Literal::StringLiteral(literal.to_string());

    assert!(match (
        string_literal.compile(Target::Python),
        string_literal.compile(Target::Javascript)
    ) {
        (Some(python), Some(javascript)) => python == literal && javascript == literal,
        _ => false,
    });
}

#[test]
fn literal_number() {
    let literal = 42;
    let number_literal = Literal::NumberLiteral(literal);

    assert!(match (
        number_literal.compile(Target::Python),
        number_literal.compile(Target::Javascript)
    ) {
        (Some(python), Some(javascript)) =>
            python == format!("{}", literal) && javascript == format!("{}", literal),
        _ => false,
    });
}
