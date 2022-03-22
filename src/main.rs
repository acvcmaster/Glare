use translation::configure_language;

mod languages;
mod parser;
mod tests;
mod token;
mod tokenizer;
mod translation;
mod codegen;

fn main() {
    configure_language();
}
