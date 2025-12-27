/// URL to the Proxmox VE download page.
pub const PROX_DL_PG_URL: &str = "https://www.proxmox.com/en/downloads/proxmox-virtual-environment";

/// Regex pattern to match Proxmox VE ISO download URLs.   
pub const ISO_URL_REGEX_PATTERN: &str =
    r#"^https://enterprise\.proxmox\.com/iso/proxmox-ve_[\d\.]+-.*\.iso$"#;
