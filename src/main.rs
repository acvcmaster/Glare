use translation::configure_language;

mod codegen;
mod languages;
mod parser;
mod tests;
mod token;
mod tokenizer;
mod translation;

fn main() {
    configure_language();
}
