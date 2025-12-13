use crate::auto_installer::commands::constants::{CommandParseError, Commands, USAGE};
use std::env;

/// Parse command from CLI arguments
pub fn parse_command() -> Result<Commands, CommandParseError> {
    parse_command_from(env::args().skip(1))
}

/// Parse command from an iterator of strings
pub fn parse_command_from<I>(mut args: I) -> Result<Commands, CommandParseError>
where
    I: Iterator<Item = String>,
{
    match args.next().as_deref() {
        Some("-h") | Some("--help") | Some("help") => {
            println!("{}", USAGE);
            Err(CommandParseError)
        }
        Some("download") => {
            let dest_path = args.next().unwrap_or_else(Commands::default_download_path);

            Ok(Commands::Download {
                dest_path: Some(dest_path),
            })
        }
        Some(cmd) => cmd.parse::<Commands>(),
        None => {
            eprintln!("{}", USAGE);
            Err(CommandParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto_installer::commands::constants::IsoType;

    #[test]
    fn test_parse_command() {
        let test_cases = vec![
            (
                vec!["download"],
                Ok(Commands::Download {
                    dest_path: Some(Commands::default_download_path()),
                }),
            ),
            (
                vec!["download", "/tmp/proxmox.iso"],
                Ok(Commands::Download {
                    dest_path: Some("/tmp/proxmox.iso".to_string()),
                }),
            ),
            (
                vec!["offline-installer"],
                Ok(Commands::AutoInstaller(IsoType::Offline)),
            ),
            (
                vec!["network-installer"],
                Ok(Commands::AutoInstaller(IsoType::Network)),
            ),
            (vec!["serve-answers"], Ok(Commands::ServeAnswers)),
            (vec!["exit"], Ok(Commands::Exit)),
            (vec!["-h"], Err(CommandParseError)),
            (vec!["--help"], Err(CommandParseError)),
            (vec![], Err(CommandParseError)),
            (vec!["invalid"], Err(CommandParseError)),
        ];

        for (args, expected) in test_cases {
            let result = parse_command_from(args.into_iter().map(String::from));
            assert_eq!(result, expected);
        }
    }
}
