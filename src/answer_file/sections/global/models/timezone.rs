use crate::answer_file::sections::global::errors::GlobalConfigError;
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashSet, fmt, str::FromStr};

static PROX_TIMEZONES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/data/timezones.txt"
    ))
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty())
    .collect()
});

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Timezone(String);

impl Timezone {
    /// Canonical string value (e.g. "America/New_York")
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Explicit constructor for non-default manipulation
    pub fn try_new(s: &str) -> Result<Self, GlobalConfigError> {
        s.parse()
    }
}

impl FromStr for Timezone {
    type Err = GlobalConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tz = s.trim();
        if PROX_TIMEZONES.contains(tz) {
            Ok(Self(tz.to_owned()))
        } else {
            Err(GlobalConfigError::Timezone)
        }
    }
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Default for Timezone {
    fn default() -> Self {
        Self("UTC".into())
    }
}

impl<'de> Deserialize<'de> for Timezone {
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
    use std::str::FromStr;

    /* ---------------- DEFAULT ---------------- */

    #[test]
    fn default_timezone_is_utc_and_valid() {
        let tz = Timezone::default();
        assert_eq!(tz.as_str(), "UTC");
        assert!(PROX_TIMEZONES.contains("UTC"));
    }

    /* ---------------- FROMSTR ---------------- */

    #[test]
    fn valid_timezone_parses() {
        let tz = Timezone::from_str("America/New_York").unwrap();
        assert_eq!(tz.as_str(), "America/New_York");
    }

    #[test]
    fn valid_timezone_with_whitespace_parses() {
        let tz = Timezone::from_str("  Europe/London ").unwrap();
        assert_eq!(tz.as_str(), "Europe/London");
    }

    #[test]
    fn invalid_timezone_fails() {
        assert_eq!(
            Timezone::from_str("Mars/Phobos"),
            Err(GlobalConfigError::Timezone)
        );
    }

    /* ---------------- TRY_NEW ---------------- */

    #[test]
    fn try_new_valid() {
        let tz = Timezone::try_new("Asia/Tokyo").unwrap();
        assert_eq!(tz.as_str(), "Asia/Tokyo");
    }

    #[test]
    fn try_new_invalid() {
        assert_eq!(
            Timezone::try_new("Invalid/Zone"),
            Err(GlobalConfigError::Timezone)
        );
    }

    /* ---------------- DISPLAY ---------------- */

    #[test]
    fn display_outputs_canonical_string() {
        let tz = Timezone::from_str("UTC").unwrap();
        assert_eq!(tz.to_string(), "UTC");
    }

    /* ---------------- SERDE ---------------- */

    #[test]
    fn serde_deserializes_valid_timezone() {
        let toml = r#"timezone = "America/Los_Angeles""#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            timezone: Timezone,
        }

        let w: Wrapper = toml::from_str(toml).unwrap();
        assert_eq!(w.timezone.as_str(), "America/Los_Angeles");
    }

    #[test]
    fn serde_rejects_invalid_timezone() {
        let toml = r#"timezone = "Invalid/Zone""#;

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Wrapper {
            timezone: Timezone,
        }

        let err = toml::from_str::<Wrapper>(toml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("timezone"),
            "unexpected error message: {}",
            msg
        );
    }
    /* ---------------- COVERAGE SANITY ---------------- */

    #[test]
    fn known_timezones_dataset_contains_common_values() {
        for tz in ["UTC", "America/New_York", "Europe/London", "Asia/Tokyo"] {
            assert!(
                PROX_TIMEZONES.contains(tz),
                "expected dataset to contain {}",
                tz
            );
        }
    }
}
