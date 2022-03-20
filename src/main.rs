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

    let a = "
        /**
         * This is a multiline comment.
         * It has multiple lines.
         */
        
        42
    ";
    let mut tokenizer = Tokenizer::new(a);
    let mut parser = Parser::new(&mut tokenizer);

    let result = parser.literal().unwrap();

    println!("{:?}", result);
}
