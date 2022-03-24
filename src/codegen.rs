use crate::parser::Literal;

pub enum Target {
    Python,
    Javascript,
}

pub trait Compilable {
    fn compile(&self, target: Target) -> Option<String>;
}

impl Compilable for Literal {
    fn compile(&self, _: Target) -> Option<String> {
        match &self {
            Literal::StringLiteral(value) => Some(value.to_string()),
            Literal::NumberLiteral(value) => Some(format!("{}", value)),
        }
    }
}

pub fn compile(node: &dyn Compilable, target: Target) -> Option<String> {
    node.compile(target)
}