use std::env;

use serde_json::{from_str, Value};
use tinytemplate::TinyTemplate;

use crate::languages::english::ENGLISH;

pub enum TranslationKey {
    UnexpectedSymbol,
    UnexpectedSymbolComplete,
    ExpectedLiteral,
    ExpectedType,
    None,
}

impl From<&TranslationKey> for &'static str {
    fn from(val: &TranslationKey) -> Self {
        match val {
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
static mut TEMPLATE_ENGINE: Option<TinyTemplate> = None;

pub fn configure_language() {
    let env_language = env::var("GLARE_LANGUAGE").unwrap_or_else(|_| "ENGLISH".to_string());

    unsafe {
        LANGUAGE = Language::from(env_language);
        TEMPLATE_ENGINE = Some(TinyTemplate::new());

        if let Some(template_engine) = &mut TEMPLATE_ENGINE {
            // templates
            let translation_keys = vec![
                TranslationKey::UnexpectedSymbol,
                TranslationKey::UnexpectedSymbolComplete,
                TranslationKey::ExpectedLiteral,
                TranslationKey::ExpectedType,
                TranslationKey::None,
            ];

            for translation_key in &translation_keys {
                template_engine
                    .add_template(translation_key.into(), get_template(translation_key))
                    .unwrap();
            }
        }
    }
}

pub fn get_template(key: &TranslationKey) -> &'static str {
    let key: &'static str = key.into();

    unsafe {
        match LANGUAGE {
            Language::English => ENGLISH[key],
            Language::Portuguese => todo!(),
            Language::Japanese => todo!(),
        }
    }
}

pub fn get_translated(key: &TranslationKey, params: &str) -> Result<String, ()> {
    let value: Value = from_str(params).unwrap();

    unsafe {
        if let Some(template_engine) = &mut TEMPLATE_ENGINE {
            return match template_engine.render(key.into(), &value) {
                Ok(result) => Ok(result),
                Err(_) => Err(()),
            };
        }
    }

    Err(())
}
