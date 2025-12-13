use std::{fmt, str::FromStr};

pub const USAGE: &str = r#"
Usage: pve-auto <command> [options]

Commands:
  download [path]     Download Proxmox VE ISO to [path], defaults to:
                        $XDG_DATA_HOME/pve-auto/proxmox-ve-latest.iso 
                        or ~/.local/share/pve-auto/proxmox-ve-latest.iso
  offline-installer   Create unattended ISO (offline; requires MGMT MAC)
  network-installer   Create unattended ISO (network; DHCP required)
  serve-answers       Start HTTP server for network installer
  exit                Exit program
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoType {
    Offline,
    Network,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    Download { dest_path: Option<String> },
    AutoInstaller(IsoType),
    ServeAnswers,
    Help,
    Exit,
}

impl Commands {
    /// Returns a user-writable default download path (XDG compliant)
    ///
    /// # Returns
    /// A `String` representing the default download path for the Proxmox VE ISO.
    /// This path is determined based on the `XDG_DATA_HOME` environment variable,
    /// falling back to `~/.local/share` if `XDG_DATA_HOME` is not set.
    pub fn default_download_path() -> String {
        if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
            format!("{}/pve-auto/proxmox-ve-latest.iso", dir)
        } else if let Ok(home) = std::env::var("HOME") {
            format!("{}/.local/share/pve-auto/proxmox-ve-latest.iso", home)
        } else {
            "proxmox-ve-latest.iso".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandParseError;

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid command")
    }
}

impl std::error::Error for CommandParseError {}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Download { .. } => write!(f, "download"),
            Commands::AutoInstaller(IsoType::Offline) => write!(f, "offline-installer"),
            Commands::AutoInstaller(IsoType::Network) => write!(f, "network-installer"),
            Commands::ServeAnswers => write!(f, "serve-answers"),
            Commands::Help => write!(f, "help"),
            Commands::Exit => write!(f, "exit"),
        }
    }
}

impl FromStr for Commands {
    type Err = CommandParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "download" => Ok(Commands::Download {
                dest_path: Some(Commands::default_download_path()),
            }),
            "offline-installer" => Ok(Commands::AutoInstaller(IsoType::Offline)),
            "network-installer" => Ok(Commands::AutoInstaller(IsoType::Network)),
            "serve-answers" => Ok(Commands::ServeAnswers),
            "exit" => Ok(Commands::Exit),
            "help" | "-h" | "--help" => Ok(Commands::Help),
            _ => Err(CommandParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_str() {
        assert_eq!(
            Commands::from_str("download").unwrap(),
            Commands::Download {
                dest_path: Some(Commands::default_download_path())
            }
        );

        assert_eq!(
            Commands::from_str("offline-installer").unwrap(),
            Commands::AutoInstaller(IsoType::Offline)
        );

        assert_eq!(
            Commands::from_str("network-installer").unwrap(),
            Commands::AutoInstaller(IsoType::Network)
        );

        assert_eq!(
            Commands::from_str("serve-answers").unwrap(),
            Commands::ServeAnswers
        );

        assert_eq!(Commands::from_str("exit").unwrap(), Commands::Exit);

        assert_eq!(Commands::from_str("help").unwrap(), Commands::Help);
        assert_eq!(Commands::from_str("-h").unwrap(), Commands::Help);
        assert_eq!(Commands::from_str("--help").unwrap(), Commands::Help);

        assert!(Commands::from_str("invalid").is_err());
    }

    #[test]
    fn test_command_to_string() {
        assert_eq!(
            Commands::Download {
                dest_path: Some(Commands::default_download_path())
            }
            .to_string(),
            "download"
        );

        assert_eq!(
            Commands::AutoInstaller(IsoType::Offline).to_string(),
            "offline-installer"
        );

        assert_eq!(
            Commands::AutoInstaller(IsoType::Network).to_string(),
            "network-installer"
        );

        assert_eq!(Commands::ServeAnswers.to_string(), "serve-answers");
        assert_eq!(Commands::Exit.to_string(), "exit");
        assert_eq!(Commands::Help.to_string(), "help");
    }
}
