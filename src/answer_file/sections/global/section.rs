use crate::answer_file::sections::global::{
    GlobalConfigError,
    constants::{EMAIL_OR_LOCALHOST_PATTERN, FQDN_PATTERN, HASHED_PASSWORD_PATTERN},
    models::{
        allowed_keyboards::KeyboardLayout, country::CountryCode, reboot_mode::RebootMode,
        timezone::Timezone,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GlobalConfig {
    pub keyboard: KeyboardLayout,
    pub country: CountryCode,
    pub timezone: Timezone,
    pub fqdn: String,
    pub mailto: String,
    #[serde(rename = "root-password-hashed")]
    /* root-password (plain text) is not supported for security reasons */
    pub root_password_hashed: String,
    #[serde(rename = "root-ssh-keys")]
    pub root_ssh_keys: Option<Vec<String>>,
    #[serde(rename = "reboot-on-error")]
    pub reboot_on_error: bool, // default false
    #[serde(rename = "reboot-mode")]
    pub reboot_mode: RebootMode, // "reboot", "power-off"
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            keyboard: KeyboardLayout::EnglishUS,
            country: CountryCode::default(),
            timezone: Timezone::default(),
            fqdn: "proxmox.lab.local".into(),
            mailto: "root@localhost".into(),
            root_ssh_keys: None,
            reboot_on_error: false,
            reboot_mode: RebootMode::default(),
            root_password_hashed: format!("$6$rounds=656000$12345678${}", "A".repeat(86)),
        }
    }
}

impl GlobalConfig {
    pub fn validate(&self) -> Result<(), GlobalConfigError> {
        if self.fqdn.len() > 255 || !FQDN_PATTERN.is_match(&self.fqdn) {
            return Err(GlobalConfigError::Fqdn);
        }

        if !EMAIL_OR_LOCALHOST_PATTERN.is_match(&self.mailto) {
            return Err(GlobalConfigError::Mailto);
        }

        if !HASHED_PASSWORD_PATTERN.is_match(&self.root_password_hashed) {
            return Err(GlobalConfigError::RootPasswordHashed);
        }

        if let Some(keys) = &self.root_ssh_keys {
            for key in keys {
                sshkeys::PublicKey::from_string(key).map_err(|_| GlobalConfigError::RootSshKeys)?;
            }
        }

        Ok(())
    }

    /* -------- FROM TOML STRING (BARE OR [global]) -------- */

    pub fn from_toml_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Try wrapped form FIRST: [global]
        if let Ok(wrapper) = toml::from_str::<Wrapper>(s) {
            wrapper.global.validate()?;
            return Ok(wrapper.global);
        }

        // 2. Fallback to bare form (preserves real errors)
        let cfg: GlobalConfig = toml::from_str(s)?;
        cfg.validate()?;
        Ok(cfg)
    }

    /* -------- TO TOML STRING (BARE OR [global]) -------- */

    pub fn to_toml_string(&self, wrap: bool) -> Result<String, Box<dyn std::error::Error>> {
        self.validate()?;

        if wrap {
            #[derive(serde::Serialize)]
            struct Wrapped<'a> {
                global: &'a GlobalConfig,
            }
            Ok(toml::to_string_pretty(&Wrapped { global: self })?)
        } else {
            Ok(toml::to_string_pretty(self)?)
        }
    }
}

#[derive(serde::Deserialize)]
struct Wrapper {
    global: GlobalConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /* ---------------- DEFAULTS ---------------- */

    #[test]
    fn defaults_are_correct() {
        let cfg = GlobalConfig::default();
        assert_eq!(cfg.keyboard, KeyboardLayout::EnglishUS);
        assert_eq!(cfg.country, CountryCode::default());
        assert_eq!(cfg.timezone, Timezone::default());
        assert_eq!(cfg.reboot_mode, RebootMode::Reboot);
        assert!(cfg.validate().is_ok());
    }

    /* ---------------- TYPE PARSING ---------------- */

    #[test]
    fn valid_keyboard_parses() {
        let kb = KeyboardLayout::from_str("en-us").unwrap();
        assert_eq!(kb, KeyboardLayout::EnglishUS);
        assert_eq!(kb.to_string(), "en-us");
    }

    #[test]
    fn invalid_keyboard_fails_at_parse() {
        assert!(KeyboardLayout::from_str("enus").is_err());
    }

    #[test]
    fn valid_reboot_mode_parses() {
        let rm = RebootMode::from_str("reboot").unwrap();
        assert_eq!(rm, RebootMode::Reboot);
        assert_eq!(rm.to_string(), "reboot");
    }

    #[test]
    fn invalid_reboot_mode_fails_at_parse() {
        assert!(RebootMode::from_str("shutdown").is_err());
    }

    #[test]
    fn valid_country_and_timezone_parse() {
        assert!(CountryCode::from_str("us").is_ok());
        assert!(Timezone::from_str("America/New_York").is_ok());
    }

    #[test]
    fn invalid_country_fails_at_parse() {
        assert_eq!(
            CountryCode::from_str("usa"),
            Err(GlobalConfigError::Country)
        );
    }

    #[test]
    fn invalid_timezone_fails_at_parse() {
        assert_eq!(
            Timezone::from_str("Mars/Phobos"),
            Err(GlobalConfigError::Timezone)
        );
    }

    /* ---------------- STRUCTURAL VALIDATION ---------------- */

    #[test]
    fn fqdn_max_length_boundary() {
        let mut cfg = GlobalConfig::default();
        let label = "a".repeat(63);
        cfg.fqdn = format!("{}.{}.{}.{}", label, label, label, label);

        assert_eq!(cfg.fqdn.len(), 255);
        assert!(cfg.validate().is_ok());

        cfg.fqdn.push('a');
        assert_eq!(cfg.validate(), Err(GlobalConfigError::Fqdn));
    }

    #[test]
    fn mailto_validation() {
        let mut cfg = GlobalConfig::default();
        cfg.mailto = "admin@example.com".into();
        assert!(cfg.validate().is_ok());

        cfg.mailto = "nope".into();
        assert_eq!(cfg.validate(), Err(GlobalConfigError::Mailto));
    }

    #[test]
    fn root_password_hash_validation() {
        let mut cfg = GlobalConfig::default();
        cfg.root_password_hashed = "nope".into();
        assert_eq!(cfg.validate(), Err(GlobalConfigError::RootPasswordHashed));
    }

    /* ---------------- SSH KEY VALIDATION ---------------- */

    #[test]
    fn valid_ssh_public_key_passes_validation() {
        let mut cfg = GlobalConfig::default();
        cfg.root_ssh_keys = Some(vec![
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIE2J8WcN6i/K3PaY5E9O+V1YxDCEV4VpWw2X2gYdEx+Z test@example"
            .to_string(),
    ]);

        assert_eq!(cfg.validate(), Ok(()));
    }

    #[test]
    fn invalid_ssh_public_key_fails_validation() {
        let mut cfg = GlobalConfig::default();
        cfg.root_ssh_keys = Some(vec!["ssh-ed25519 NOT_A_REAL_KEY test@example".to_string()]);

        assert_eq!(cfg.validate(), Err(GlobalConfigError::RootSshKeys));
    }

    #[test]
    fn multiple_ssh_keys_fail_on_first_invalid() {
        let mut cfg = GlobalConfig::default();
        cfg.root_ssh_keys = Some(vec![
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIB6sR1zF9Q3y6Jv5k1p2b3c4d5e6f7g8h9i0j test@example"
                .to_string(),
            "ssh-ed25519 INVALIDKEY test2@example".to_string(),
        ]);

        assert_eq!(cfg.validate(), Err(GlobalConfigError::RootSshKeys));
    }

    #[test]
    fn no_ssh_keys_is_valid() {
        let mut cfg = GlobalConfig::default();
        cfg.root_ssh_keys = None;

        assert_eq!(cfg.validate(), Ok(()));
    }

    /* ---------------- TOML DESERIALIZATION ---------------- */

    #[test]
    fn valid_toml_deserialization_bare() {
        let toml = r#"
            keyboard = "en-us"
            country = "us"
            timezone = "America/New_York"
            fqdn = "proxmox.lab.local"
            mailto = "root@localhost"
            reboot_on_error = false
            reboot_mode = "reboot"
            root-password-hashed = "$6$rounds=656000$12345678$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        "#;

        let cfg = GlobalConfig::from_toml_str(toml).unwrap();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn valid_toml_deserialization_wrapped() {
        let toml = r#"
            [global]
            keyboard = "en-us"
            country = "us"
            timezone = "America/New_York"
            fqdn = "proxmox.lab.local"
            mailto = "root@localhost"
            reboot_on_error = false
            reboot_mode = "reboot"
            root-password-hashed = "$6$rounds=656000$12345678$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        "#;

        let cfg = GlobalConfig::from_toml_str(toml).unwrap();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn invalid_toml_enum_fails_at_deserialize() {
        let toml = r#"
            keyboard = "enus"
            country = "us"
            timezone = "America/New_York"
            fqdn = "proxmox.lab.local"
            mailto = "root@localhost"
            reboot_on_error = false
            reboot_mode = "reboot"
            root-password-hashed = "$6$rounds=656000$12345678$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        "#;

        let err = GlobalConfig::from_toml_str(toml).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("keyboard"), "error was: {}", msg);
    }

    /* ---------------- TOML SERIALIZATION ---------------- */

    #[test]
    fn toml_round_trip_bare() {
        let cfg = GlobalConfig::default();
        let toml = cfg.to_toml_string(false).unwrap();
        let parsed = GlobalConfig::from_toml_str(&toml).unwrap();
        assert_eq!(cfg, parsed);
    }

    #[test]
    fn toml_round_trip_wrapped() {
        let cfg = GlobalConfig::default();
        let toml = cfg.to_toml_string(true).unwrap();
        let parsed = GlobalConfig::from_toml_str(&toml).unwrap();
        assert_eq!(cfg, parsed);
    }
}
