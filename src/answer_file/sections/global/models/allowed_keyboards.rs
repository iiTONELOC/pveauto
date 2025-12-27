use crate::answer_file::macros::string_enum;
use crate::answer_file::sections::global::errors::GlobalConfigError;
/* ===================== KEYBOARD LAYOUT ===================== */
string_enum!(
    #[derive(Debug, Clone,   PartialEq, Eq, Hash)]
    pub enum KeyboardLayout {
        German => "de",
        GermanSwiss => "de-ch",
        Danish => "dk",
        EnglishUK => "en-gb",
        EnglishUS => "en-us",
        Spanish => "es",
        Finnish => "fi",
        French => "fr",
        FrenchBelgium => "fr-be",
        FrenchCanada => "fr-ca",
        FrenchSwiss => "fr-ch",
        Hungarian => "hu",
        Icelandic => "is",
        Italian => "it",
        Japanese => "jp",
        Lithuanian => "lt",
        Macedonian => "mk",
        Dutch => "nl",
        Norwegian => "no",
        Polish => "pl",
        Portuguese => "pt",
        PortugueseBrazil => "pt-br",
        Swedish => "se",
        Slovenian => "si",
        Turkish => "tr",
    },
    GlobalConfigError,
    GlobalConfigError::Keyboard
);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    /* ---------------- FROMSTR ---------------- */

    #[test]
    fn valid_keyboard_parses() {
        let k = KeyboardLayout::from_str("en-us").unwrap();
        assert_eq!(k, KeyboardLayout::EnglishUS);
    }

    #[test]
    fn valid_keyboard_round_trip_string() {
        let k = KeyboardLayout::from_str("fr-ca").unwrap();
        assert_eq!(k.to_string(), "fr-ca");
    }

    #[test]
    fn invalid_keyboard_fails_at_parse() {
        assert_eq!(
            KeyboardLayout::from_str("enus"),
            Err(GlobalConfigError::Keyboard)
        );
    }

    /* ---------------- DISPLAY ---------------- */

    #[test]
    fn display_outputs_canonical_string() {
        assert_eq!(KeyboardLayout::German.to_string(), "de");
        assert_eq!(KeyboardLayout::PortugueseBrazil.to_string(), "pt-br");
    }

    /* ---------------- SERDE ---------------- */

    #[test]
    fn serde_deserializes_valid_keyboard() {
        let toml = r#"keyboard = "en-gb""#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            keyboard: KeyboardLayout,
        }

        let w: Wrapper = toml::from_str(toml).unwrap();
        assert_eq!(w.keyboard, KeyboardLayout::EnglishUK);
    }

    #[test]
    fn serde_rejects_invalid_keyboard() {
        let toml = r#"keyboard = "english-us""#;

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Wrapper {
            keyboard: KeyboardLayout,
        }

        let err = toml::from_str::<Wrapper>(toml).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("keyboard"), "error was: {}", msg);
    }

    /* ---------------- ROUND TRIP ---------------- */

    #[test]
    fn serde_round_trip_preserves_string_value() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Wrapper {
            keyboard: KeyboardLayout,
        }

        let w = Wrapper {
            keyboard: KeyboardLayout::FrenchCanada,
        };

        let toml = toml::to_string(&w).unwrap();
        assert!(toml.contains(r#"keyboard = "fr-ca""#));

        let parsed: Wrapper = toml::from_str(&toml).unwrap();
        assert_eq!(parsed, w);
    }

    /* ---------------- COVERAGE SANITY ---------------- */

    #[test]
    fn common_keyboards_parse() {
        for (s, expected) in [
            ("de", KeyboardLayout::German),
            ("en-us", KeyboardLayout::EnglishUS),
            ("pt-br", KeyboardLayout::PortugueseBrazil),
            ("jp", KeyboardLayout::Japanese),
        ] {
            assert_eq!(KeyboardLayout::from_str(s).unwrap(), expected);
        }
    }
}
