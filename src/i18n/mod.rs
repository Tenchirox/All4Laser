mod fr;
mod ja;
mod de;
mod it;
mod es;
mod pt;
mod ar;
mod zh;
mod ru;
mod tr;
mod ko;
mod pl;

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

// Global localization store — each language is loaded from its own submodule
static DICTIONARY: LazyLock<HashMap<Language, HashMap<&'static str, &'static str>>> = LazyLock::new(
    || {
        let mut m = HashMap::new();

        let mut en_map = HashMap::new();
        en_map.insert("Preferences", "Preferences");
        en_map.insert("Appearance", "Appearance");
        en_map.insert("General", "General");
        en_map.insert("Accessibility", "Accessibility");
        en_map.insert("Advanced", "Advanced");
        en_map.insert("Appearance Settings", "Appearance Settings");
        en_map.insert("Language Settings", "Language Settings");
        en_map.insert("General Settings", "General Settings");
        en_map.insert("Accessibility Settings", "Accessibility Settings");
        en_map.insert("Advanced Settings", "Advanced Settings");
        en_map.insert("Display Units", "Display Units");
        en_map.insert("Millimeters", "Millimeters");
        en_map.insert("Inches", "Inches");
        en_map.insert("Max Undo Steps", "Max Undo Steps");
        en_map.insert("Colorblind Mode", "Colorblind Mode");
        en_map.insert("High Contrast", "High Contrast");
        en_map.insert("Darkroom Mode", "Darkroom Mode");
        en_map.insert("Touch Mode", "Touch Mode");
        en_map.insert("Watch Folder for Auto-import", "Watch Folder for Auto-import");
        en_map.insert("Apply", "Apply");
        en_map.insert("Cancel", "Cancel");
        en_map.insert("OK", "OK");
        en_map.insert("None", "None");
        en_map.insert("Protanopia", "Protanopia");
        en_map.insert("Deuteranopia", "Deuteranopia");
        en_map.insert("Tritanopia", "Tritanopia");
        en_map.insert("Machine", "Machine");
        en_map.insert("Machine Settings", "Machine Settings");
        en_map.insert("Workspace Size", "Workspace Size");
        en_map.insert("Width (mm)", "Width (mm)");
        en_map.insert("Height (mm)", "Height (mm)");
        en_map.insert("Optimization", "Optimization");
        en_map.insert("Enable Automatic Optimization", "Enable Automatic Optimization");
        en_map.insert("Automatically optimize tool paths and nesting for better efficiency", "Automatically optimize tool paths and nesting for better efficiency");
        en_map.insert("Updates", "Updates");
        en_map.insert("Enable Automatic Updates", "Enable Automatic Updates");
        en_map.insert("Automatically check and install updates in the background", "Automatically check and install updates in the background");
        en_map.insert("Note: These changes will be applied to your machine profile", "Note: These changes will be applied to your machine profile");
        en_map.insert("Auto-Optimization", "Auto-Optimization");
        m.insert(Language::English, en_map);
        m.insert(Language::French, fr::entries());
        m.insert(Language::Japanese, ja::entries());
        m.insert(Language::German, de::entries());
        m.insert(Language::Italian, it::entries());
        m.insert(Language::Spanish, es::entries());
        m.insert(Language::Portuguese, pt::entries());
        m.insert(Language::Arabic, ar::entries());
        m.insert(Language::Chinese, zh::entries());
        m.insert(Language::Russian, ru::entries());
        m.insert(Language::Turkish, tr::entries());
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
