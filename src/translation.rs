use std::env;

use crate::languages::english::ENGLISH;

pub enum TranslationKey {
    UnexpectedSymbol,
    UnexpectedSymbolComplete,
    ExpectedLiteral,
    ExpectedType,
    None,
}

impl From<TranslationKey> for &'static str {
    fn from(val: TranslationKey) -> Self {
        match &val {
            TranslationKey::UnexpectedSymbol => "UnexpectedSymbol",
            TranslationKey::UnexpectedSymbolComplete => "UnexpectedSymbolComplete",
            TranslationKey::ExpectedLiteral => "ExpectedLiteral",
            TranslationKey::ExpectedType => "ExpectedType",
            TranslationKey::None => "None",
        }
    }
}

pub enum Language {
    English,
    Portuguese,
    Japanese,
}

impl From<String> for Language {
    fn from(string: String) -> Self {
        match string.as_str() {
            "Portuguese" | "Português" => Language::Portuguese,
            "Japanese" | "日本語" => Language::Japanese,
            _ => Language::English,
        }
    }
}

static mut LANGUAGE: Language = Language::English;

pub fn configure_language() {
    if let Ok(env_language) = env::var("GLARE_LANGUAGE") {
        unsafe {
            LANGUAGE = Language::from(env_language);
        }
    }
}

pub fn get_translated(key: TranslationKey) -> &'static str {
    let key: &'static str = key.into();

    unsafe {
        match LANGUAGE {
            Language::English => ENGLISH[key],
            Language::Portuguese => todo!(),
            Language::Japanese => todo!(),
        }
    }
}
