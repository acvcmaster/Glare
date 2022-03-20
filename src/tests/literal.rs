#[cfg(test)]
mod literal_tests {
    use crate::{parser::Parser, tokenizer::Tokenizer};

    #[test]
    fn string_literal_ok() {
        let mut tokenizer = Tokenizer::new("`Literal string`");
        let mut parser = Parser::new(&mut tokenizer);

        assert!(parser.parse_literal(true).is_ok())
    }

    #[test]
    fn number_literal_ok() {
        let mut tokenizer = Tokenizer::new("42");
        let mut parser = Parser::new(&mut tokenizer);

        assert!(parser.parse_literal(true).is_ok())
    }

    #[test]
    fn literal_err() {
        let mut tokenizer = Tokenizer::new("# Comment");
        let mut parser = Parser::new(&mut tokenizer);

        assert!(parser.parse_literal(true).is_err())
    }
}
