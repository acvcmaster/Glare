use phf::{self, phf_map, Map};

pub static ENGLISH: Map<&'static str, &'static str> = phf_map! {
    "UnexpectedSymbol" => "Unexpected symbol at index {index}",
    "UnexpectedSymbolComplete" => "Unexpected symbol: {symbol} (at index {index})",
    "ExpectedLiteral" => "Expected literal",
    "ExpectedType" => "Expected valid type",
    "None" => "None",
};
