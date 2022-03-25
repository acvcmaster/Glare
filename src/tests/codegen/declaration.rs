use crate::{
    codegen::{compile, Target},
    parser::{Declaration, Literal, Variable},
};

#[test]
fn declaration() {
    let declaration = Declaration {
        variable: Variable {
            name: "test".to_string(),
        },
        union_type: None,
        literal: Literal::StringLiteral("Test variable".to_string()),
    };

    assert!(match (
        compile(&declaration, Target::Python),
        compile(&declaration, Target::Javascript),
    ) {
        (Some(python), Some(javascript)) =>
            python == "test = \"Test variable\"" && javascript == "let test = `Test variable`;",
        _ => false,
    });
}
