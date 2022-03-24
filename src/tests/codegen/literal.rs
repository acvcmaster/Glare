use crate::{
    codegen::{compile, Target},
    parser::Literal,
};

#[test]
fn literal_string() {
    let literal = "String Literal";
    let string_literal = Literal::StringLiteral(literal.to_string());

    assert!(match (
        compile(&string_literal, Target::Python),
        compile(&string_literal, Target::Javascript),
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
        compile(&number_literal, Target::Python),
        compile(&number_literal, Target::Javascript),
    ) {
        (Some(python), Some(javascript)) =>
            python == format!("{}", literal) && javascript == format!("{}", literal),
        _ => false,
    });
}
