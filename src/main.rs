use parser::Parser;
use translation::configure_language;

use crate::tokenizer::Tokenizer;

mod languages;
mod parser;
mod token;
mod tokenizer;
mod translation;

fn main() {
    configure_language();

    let a = r#"
        # This is a comment

        "Hello, planet!"
    "#;
    let mut tokenizer = Tokenizer::new(a);
    let mut parser = Parser::new(&mut tokenizer);

    match parser.parse_pipe(true) {
        Ok(result) => println!("{:?}", result),
        Err(error) => println!("{}", error),
    }
}
