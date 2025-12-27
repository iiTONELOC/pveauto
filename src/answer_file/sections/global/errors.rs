use crate::answer_file::macros::config_error_enum;

/* ===================== GLOBAL CONFIG ERROR ===================== */

config_error_enum!(
    #[derive(Debug, PartialEq)]
    pub enum GlobalConfigError {
        Keyboard => "keyboard.invalid_format",
        Country => "country.invalid_format",
        Timezone => "timezone.invalid_format",
        Fqdn => "fqdn.invalid_format",
        Mailto => "mailto.invalid_format",
        RootSshKeys => "root_ssh_keys.invalid_format",
        RootPasswordHashed => "root_password_hashed.invalid_format",
        RebootOnError => "reboot_on_error.invalid_format",
        RebootMode => "reboot_mode.invalid_format",
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    /* ---------------- CODE MAPPING ---------------- */

    #[test]
    fn keyboard_error_code() {
        assert_eq!(
            GlobalConfigError::Keyboard.code(),
            "keyboard.invalid_format"
        );
    }

    #[test]
    fn country_error_code() {
        assert_eq!(GlobalConfigError::Country.code(), "country.invalid_format");
    }

    #[test]
    fn timezone_error_code() {
        assert_eq!(
            GlobalConfigError::Timezone.code(),
            "timezone.invalid_format"
        );
    }

    #[test]
    fn fqdn_error_code() {
        assert_eq!(GlobalConfigError::Fqdn.code(), "fqdn.invalid_format");
    }

    #[test]
    fn mailto_error_code() {
        assert_eq!(GlobalConfigError::Mailto.code(), "mailto.invalid_format");
    }

    #[test]
    fn root_ssh_keys_error_code() {
        assert_eq!(
            GlobalConfigError::RootSshKeys.code(),
            "root_ssh_keys.invalid_format"
        );
    }

    #[test]
    fn root_password_hashed_error_code() {
        assert_eq!(
            GlobalConfigError::RootPasswordHashed.code(),
            "root_password_hashed.invalid_format"
        );
    }

    #[test]
    fn reboot_on_error_error_code() {
        assert_eq!(
            GlobalConfigError::RebootOnError.code(),
            "reboot_on_error.invalid_format"
        );
    }

    #[test]
    fn reboot_mode_error_code() {
        assert_eq!(
            GlobalConfigError::RebootMode.code(),
            "reboot_mode.invalid_format"
        );
    }

    /* ---------------- DISPLAY ---------------- */

    #[test]
    fn display_outputs_error_code() {
        let err = GlobalConfigError::Country;
        assert_eq!(err.to_string(), "country.invalid_format");
    }

    /* ---------------- ERROR TRAIT ---------------- */

    #[test]
    fn implements_std_error() {
        let err: &dyn Error = &GlobalConfigError::Fqdn;
        assert_eq!(err.to_string(), "fqdn.invalid_format");
    }

    /* ---------------- EQUALITY ---------------- */

    #[test]
    fn errors_are_comparable() {
        assert_eq!(GlobalConfigError::Keyboard, GlobalConfigError::Keyboard);
        assert_ne!(GlobalConfigError::Keyboard, GlobalConfigError::Country);
    }
}
