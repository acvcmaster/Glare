use parser::Parser;
use translation::configure_language;

use crate::tokenizer::Tokenizer;

mod token;
mod tokenizer;
mod translation;
mod languages;
mod parser;

fn main() {
    configure_language();
    let a = "            42          ";
    let mut tokenizer = Tokenizer::new(a);
    let mut parser = Parser::new(&mut tokenizer);

    let result = parser.literal().unwrap();

    println!("{:?}", result);
}
