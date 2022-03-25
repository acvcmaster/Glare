use crate::parser::{Declaration, Literal, Variable};

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

impl Compilable for Declaration {
    fn compile(&self, target: Target) -> Option<String> {
        match (&self.variable, &self.union_type) {
            (Variable { name }, ..) => match target {
                Target::Python => Some(format!(
                    "{} = {}",
                    *name,
                    match &self.literal {
                        Literal::StringLiteral(literal) => format!("\"{}\"", literal),
                        Literal::NumberLiteral(literal) => format!("{}", literal),
                    }
                )),
                Target::Javascript => Some(format!(
                    "let {} = {};",
                    *name,
                    match &self.literal {
                        Literal::StringLiteral(literal) => format!("`{}`", literal),
                        Literal::NumberLiteral(literal) => format!("{}", literal),
                    }
                )),
            },
            _ => None,
        }
    }
}

pub fn compile(node: &dyn Compilable, target: Target) -> Option<String> {
    node.compile(target)
}
