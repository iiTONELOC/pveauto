use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_OR_LOCALHOST_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$)|^(root|admin|user)@localhost$")
        .expect("invalid EMAIL_OR_LOCALHOST_PATTERN")
});

pub static FQDN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,63}$")
        .expect("invalid FQDN_PATTERN")
});

pub static HASHED_PASSWORD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\$6\$rounds=\d{6}\$[./A-Za-z0-9]{1,16}\$[./A-Za-z0-9]{86}$")
        .expect("invalid HASHED_PASSWORD_PATTERN")
});

#[cfg(test)]
mod tests {
    use super::*;

    /* ---------------- EMAIL / LOCALHOST ---------------- */

    #[test]
    fn valid_email_addresses_match() {
        for email in [
            "user@example.com",
            "admin@test.co",
            "first.last+tag@domain.io",
        ] {
            assert!(
                EMAIL_OR_LOCALHOST_PATTERN.is_match(email),
                "expected valid email to match: {}",
                email
            );
        }
    }

    #[test]
    fn valid_localhost_addresses_match() {
        for email in ["root@localhost", "admin@localhost", "user@localhost"] {
            assert!(
                EMAIL_OR_LOCALHOST_PATTERN.is_match(email),
                "expected localhost email to match: {}",
                email
            );
        }
    }

    #[test]
    fn invalid_email_addresses_fail() {
        for email in ["not-an-email", "user@", "@example.com", "root@local"] {
            assert!(
                !EMAIL_OR_LOCALHOST_PATTERN.is_match(email),
                "expected invalid email to fail: {}",
                email
            );
        }
    }
    /* ---------------- FQDN ---------------- */

    #[test]
    fn valid_fqdns_match() {
        for fqdn in [
            "example.com",
            "sub.domain.example",
            "proxmox.lab.local",
            "aa.bb.cc",
        ] {
            assert!(
                FQDN_PATTERN.is_match(fqdn),
                "expected valid fqdn to match: {}",
                fqdn
            );
        }
    }
    #[test]
    fn invalid_fqdns_fail() {
        for fqdn in [
            "-example.com",
            "example-.com",
            "example",
            ".example.com",
            "example..com",
            "exa_mple.com",
        ] {
            assert!(
                !FQDN_PATTERN.is_match(fqdn),
                "expected invalid fqdn to fail: {}",
                fqdn
            );
        }
    }

    /* ---------------- HASHED PASSWORD ---------------- */

    #[test]
    fn valid_hashed_password_matches() {
        let hash = format!("$6$rounds=656000$12345678${}", "A".repeat(86));
        assert!(
            HASHED_PASSWORD_PATTERN.is_match(&hash),
            "expected valid hash to match"
        );
    }

    #[test]
    fn invalid_hashed_password_fails() {
        for hash in [
            "password",
            "$6$rounds=656000$short$hash",
            "$1$rounds=656000$12345678$invalid",
        ] {
            assert!(
                !HASHED_PASSWORD_PATTERN.is_match(hash),
                "expected invalid hash to fail: {}",
                hash
            );
        }
    }
}
