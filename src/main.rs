use translation::configure_language;

use crate::tokenizer::Tokenizer;

mod token;
mod tokenizer;
mod translation;
mod languages;

fn main() {
    configure_language();
    let a = r#""Test string with double quotes""#;
    let mut tokenizer = Tokenizer::new(a);

    match tokenizer.get_next_token() {
        Ok(token) => println!("{:?}", token),
        Err(error) => println!("{}", error),
    }
}
