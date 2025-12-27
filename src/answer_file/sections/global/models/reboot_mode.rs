use crate::answer_file::macros::string_enum;

/* ===================== REBOOT MODE ===================== */
string_enum!(
    #[derive(Debug, Clone,  PartialEq, Eq, Hash)]
    pub enum RebootMode {
        Reboot => "reboot",
        PowerOff => "power-off",
    },
    crate::answer_file::sections::global::errors::GlobalConfigError,
    crate::answer_file::sections::global::errors::GlobalConfigError::RebootMode
);

impl Default for RebootMode {
    fn default() -> Self {
        RebootMode::Reboot
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    /* ---------------- DEFAULT ---------------- */

    #[test]
    fn default_is_reboot() {
        assert_eq!(RebootMode::default(), RebootMode::Reboot);
    }

    /* ---------------- FROMSTR ---------------- */

    #[test]
    fn valid_reboot_mode_parses() {
        let m = RebootMode::from_str("reboot").unwrap();
        assert_eq!(m, RebootMode::Reboot);
    }

    #[test]
    fn valid_poweroff_parses() {
        let m = RebootMode::from_str("power-off").unwrap();
        assert_eq!(m, RebootMode::PowerOff);
    }

    #[test]
    fn invalid_reboot_mode_fails_at_parse() {
        assert!(RebootMode::from_str("shutdown").is_err());
    }

    /* ---------------- DISPLAY ---------------- */

    #[test]
    fn display_outputs_canonical_string() {
        assert_eq!(RebootMode::Reboot.to_string(), "reboot");
        assert_eq!(RebootMode::PowerOff.to_string(), "power-off");
    }

    /* ---------------- SERDE ---------------- */

    #[test]
    fn serde_deserializes_valid_value() {
        let toml = r#"mode = "reboot""#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            mode: RebootMode,
        }

        let w: Wrapper = toml::from_str(toml).unwrap();
        assert_eq!(w.mode, RebootMode::Reboot);
    }

    #[test]
    fn serde_rejects_invalid_value() {
        let toml = r#"mode = "shutdown""#;

        #[derive(Debug, Deserialize)]
        #[allow(dead_code)]
        struct Wrapper {
            mode: RebootMode,
        }

        let err = toml::from_str::<Wrapper>(toml).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("reboot_mode"), "error was: {}", msg);
    }

    #[test]
    fn serde_round_trip_preserves_string_value() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Wrapper {
            mode: RebootMode,
        }

        let w = Wrapper {
            mode: RebootMode::PowerOff,
        };

        let toml = toml::to_string(&w).unwrap();
        assert!(toml.contains(r#"mode = "power-off""#));

        let parsed: Wrapper = toml::from_str(&toml).unwrap();
        assert_eq!(parsed, w);
    }
}
