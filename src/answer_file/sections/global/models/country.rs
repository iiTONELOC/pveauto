use crate::answer_file::sections::global::errors::GlobalConfigError;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    str::FromStr,
};

static PROX_COUNTRY_NAME_TO_CODE: Lazy<HashMap<String, String>> = Lazy::new(|| {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/data/country_codes.txt"
    ))
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty())
    .filter_map(|line| {
        let (name, code) = line.split_once(':')?;
        Some((name.trim().to_owned(), code.trim().to_lowercase()))
    })
    .collect()
});

static PROX_COUNTRY_CODES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    PROX_COUNTRY_NAME_TO_CODE
        .values()
        .map(|s| s.as_str())
        .collect()
});

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct CountryCode(String);

impl CountryCode {
    /// Canonical string value (e.g. "us")
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Explicit constructor for non-default manipulation
    pub fn try_new(s: &str) -> Result<Self, GlobalConfigError> {
        s.parse()
    }
}

impl FromStr for CountryCode {
    type Err = GlobalConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.trim().to_lowercase();
        if PROX_COUNTRY_CODES.contains(code.as_str()) {
            Ok(Self(code))
        } else {
            Err(GlobalConfigError::Country)
        }
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Default for CountryCode {
    fn default() -> Self {
        Self("us".into())
    }
}

impl<'de> Deserialize<'de> for CountryCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    /* ---------------- DEFAULT ---------------- */

    #[test]
    fn default_country_is_us_and_valid() {
        let c = CountryCode::default();
        assert_eq!(c.as_str(), "us");
        assert!(PROX_COUNTRY_CODES.contains("us"));
    }

    /* ---------------- FROMSTR ---------------- */

    #[test]
    fn valid_country_parses_lowercase() {
        let c = CountryCode::from_str("us").unwrap();
        assert_eq!(c.as_str(), "us");
    }

    #[test]
    fn valid_country_parses_uppercase_and_normalizes() {
        let c = CountryCode::from_str("US").unwrap();
        assert_eq!(c.as_str(), "us");
    }

    #[test]
    fn valid_country_with_whitespace_parses() {
        let c = CountryCode::from_str("  us ").unwrap();
        assert_eq!(c.as_str(), "us");
    }

    #[test]
    fn invalid_country_fails() {
        assert_eq!(
            CountryCode::from_str("usa"),
            Err(GlobalConfigError::Country)
        );
    }

    /* ---------------- TRY_NEW ---------------- */

    #[test]
    fn try_new_valid() {
        let c = CountryCode::try_new("gb").unwrap();
        assert_eq!(c.as_str(), "gb");
    }

    #[test]
    fn try_new_invalid() {
        assert_eq!(
            CountryCode::try_new("invalid"),
            Err(GlobalConfigError::Country)
        );
    }

    /* ---------------- DISPLAY ---------------- */

    #[test]
    fn display_outputs_canonical_string() {
        let c = CountryCode::from_str("US").unwrap();
        assert_eq!(c.to_string(), "us");
    }

    /* ---------------- SERDE ---------------- */

    #[test]
    fn serde_deserializes_valid_country() {
        let toml = r#"country = "ca""#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            country: CountryCode,
        }

        let w: Wrapper = toml::from_str(toml).unwrap();
        assert_eq!(w.country.as_str(), "ca");
    }

    #[test]
    fn serde_rejects_invalid_country() {
        let toml = r#"country = "usa""#;

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Wrapper {
            country: CountryCode,
        }

        let err = toml::from_str::<Wrapper>(toml).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("country"), "unexpected error message: {}", msg);
    }

    /* ---------------- ROUND TRIP ---------------- */

    #[test]
    fn serde_round_trip_preserves_canonical_value() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Wrapper {
            country: CountryCode,
        }

        let w = Wrapper {
            country: CountryCode::from_str("DE").unwrap(),
        };

        let toml = toml::to_string(&w).unwrap();
        assert!(toml.contains(r#"country = "de""#));

        let parsed: Wrapper = toml::from_str(&toml).unwrap();
        assert_eq!(parsed, w);
    }

    /* ---------------- DATASET SANITY ---------------- */

    #[test]
    fn dataset_contains_common_country_codes() {
        for code in ["us", "gb", "de", "fr", "ca"] {
            assert!(
                PROX_COUNTRY_CODES.contains(code),
                "expected dataset to contain {}",
                code
            );
        }
    }
}
