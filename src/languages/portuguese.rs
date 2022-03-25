use phf::{self, phf_map, Map};

pub static PORTUGUESE: Map<&'static str, &'static str> = phf_map! {
    "UnexpectedSymbol" => "Símbolo inesperado (offset: {index})",
    "UnexpectedSymbolComplete" => "Símbolo {symbol} inesperado (offset: {index})",
    "ExpectedLiteral" => "Literal era esperado",
    "ExpectedType" => "Tipo válido era esperado",
    "None" => "Nenhum",
};
