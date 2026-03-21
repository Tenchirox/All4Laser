mod ar;
mod de;
mod es;
mod fr;
mod it;
mod ja;
mod ko;
mod pl;
mod pt;
mod ru;
mod tr_lang;
mod zh;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    German,
    Italian,
    Arabic,
    Spanish,
    Portuguese,
    Chinese,
    Russian,
    Turkish,
    Korean,
    Polish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Japanese => "日本語",
            Language::German => "Deutsch",
            Language::Italian => "Italiano",
            Language::Arabic => "العربية",
            Language::Spanish => "Español",
            Language::Portuguese => "Português",
            Language::Chinese => "中文",
            Language::Russian => "Русский",
            Language::Turkish => "Türkçe",
            Language::Korean => "한국어",
            Language::Polish => "Polski",
        }
    }
}

// Global localization store
static DICTIONARY: LazyLock<HashMap<Language, HashMap<&'static str, &'static str>>> = LazyLock::new(
    || {
        let mut m = HashMap::new();

        // English (Base)
        m.insert(Language::English, HashMap::new());

        m.insert(Language::French, fr::entries());
        m.insert(Language::Japanese, ja::entries());
        m.insert(Language::German, de::entries());
        m.insert(Language::Italian, it::entries());
        m.insert(Language::Spanish, es::entries());
        m.insert(Language::Portuguese, pt::entries());
        m.insert(Language::Arabic, ar::entries());
        m.insert(Language::Chinese, zh::entries());
        m.insert(Language::Russian, ru::entries());
        m.insert(Language::Turkish, tr_lang::entries());
        m.insert(Language::Korean, ko::entries());
        m.insert(Language::Polish, pl::entries());

        m
    },
);

static CURRENT_LANG: LazyLock<RwLock<Language>> = LazyLock::new(|| RwLock::new(Language::English));

pub fn set_language(lang: Language) {
    if let Ok(mut l) = CURRENT_LANG.write() {
        *l = lang;
    }
}

pub fn get_language() -> Language {
    if let Ok(l) = CURRENT_LANG.read() {
        *l
    } else {
        Language::English
    }
}

pub fn tr(key: &str) -> String {
    let lang = get_language();
    if lang == Language::English {
        return key.to_string();
    }

    if let Some(map) = DICTIONARY.get(&lang) {
        let map: &HashMap<&'static str, &'static str> = map;
        if let Some(val) = map.get(key) {
            return val.to_string();
        }
    }
    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_returns_key_as_is() {
        set_language(Language::English);
        assert_eq!(tr("Connect"), "Connect");
        assert_eq!(tr("NonExistentKey"), "NonExistentKey");
    }

    #[test]
    fn french_translates_known_key() {
        let map = DICTIONARY.get(&Language::French).unwrap();
        assert_eq!(*map.get("Connect").unwrap(), "Connecter");
        assert_eq!(*map.get("Open").unwrap(), "Ouvrir");
    }

    #[test]
    fn unknown_key_falls_back_to_english() {
        set_language(Language::German);
        assert_eq!(tr("SomeUnknownKey"), "SomeUnknownKey");
        set_language(Language::English);
    }

    #[test]
    fn all_languages_have_dictionary_entries() {
        let languages = [
            Language::French,
            Language::Japanese,
            Language::German,
            Language::Italian,
            Language::Spanish,
            Language::Portuguese,
            Language::Arabic,
            Language::Chinese,
            Language::Russian,
            Language::Turkish,
            Language::Korean,
            Language::Polish,
        ];
        for lang in languages {
            assert!(
                DICTIONARY.contains_key(&lang),
                "Missing dictionary for {:?}",
                lang
            );
            let map = DICTIONARY.get(&lang).unwrap();
            assert!(
                map.contains_key("Connect"),
                "Missing 'Connect' key for {:?}",
                lang
            );
        }
    }

    #[test]
    fn all_languages_have_new_ui_keys() {
        let required_keys = [
            "Node Editing", "Selection", "Create", "Modify",
            "Rect", "Circle", "Origin X:", "Radius:", "Layer:",
            "Set to Active Layer", "Text Tool", "Variable Text (Serial Numbers)",
            "Text:", "Size:", "Source:", "Bundled", "System", "Font:",
            "Load Font File", "Add Text to Drawing",
            "Serial", "CSV Column", "Prefix:", "Suffix:", "Start:", "Inc:", "Pad:",
            "Batch Count:", "Column:", "Header row", "Delimiter:", "Load CSV",
            "Align / Distribute", "Align Left", "Align Right", "Align Top", "Align Bottom",
            "Center Horizontal", "Center Vertical", "Distribute H", "Distribute V",
            "Shape Properties", "Select a shape to edit properties.",
            "Session Recovery", "Restore", "Discard",
            "Load a GCode file or draw shapes to preview",
            "Apply Recommended", "Apply to Active Layer", "Material Presets",
            "Mode", "Spd/Pwr", "Out",
            "Array", "Grid", "Offset", "Boolean",
            "Circular Array", "Grid Array", "Offset Path", "Boolean Operations",
        ];
        let languages = [
            Language::French, Language::Japanese, Language::German,
            Language::Italian, Language::Spanish, Language::Portuguese,
            Language::Arabic, Language::Chinese, Language::Russian,
            Language::Turkish, Language::Korean, Language::Polish,
        ];
        for lang in languages {
            let map = DICTIONARY.get(&lang).expect(&format!("Missing dictionary for {:?}", lang));
            for key in &required_keys {
                assert!(
                    map.contains_key(key),
                    "Missing key '{}' for {:?}",
                    key, lang
                );
            }
        }
    }

    #[test]
    fn language_name_returns_native_name() {
        assert_eq!(Language::French.name(), "Français");
        assert_eq!(Language::Japanese.name(), "日本語");
        assert_eq!(Language::English.name(), "English");
    }

    #[test]
    fn set_and_get_language_roundtrips() {
        set_language(Language::Spanish);
        assert_eq!(get_language(), Language::Spanish);
        set_language(Language::English);
        assert_eq!(get_language(), Language::English);
    }
}
