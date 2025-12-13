use crate::auto_installer::{
    commands::constants::{Commands, IsoType},
    dispatcher::actions::DispatchAction,
};

pub mod actions;
pub mod execute;

pub fn dispatch(cmd: Commands) -> DispatchAction {
    match cmd {
        Commands::Download { dest_path } => DispatchAction::Download { dest_path },
        Commands::AutoInstaller(IsoType::Offline) => DispatchAction::AutoInstallerOffline,
        Commands::AutoInstaller(IsoType::Network) => DispatchAction::AutoInstallerNetwork,
        Commands::ServeAnswers => DispatchAction::ServeAnswers,
        Commands::Help => DispatchAction::Help,
        Commands::Exit => DispatchAction::Exit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto_installer::commands::constants::{Commands, IsoType};

    #[test]
    fn test_dispatch() {
        let test_cases = vec![
            (
                Commands::Download {
                    dest_path: Some(Commands::default_download_path()),
                },
                DispatchAction::Download {
                    dest_path: Some(Commands::default_download_path()),
                },
            ),
            (
                Commands::AutoInstaller(IsoType::Offline),
                DispatchAction::AutoInstallerOffline,
            ),
            (
                Commands::AutoInstaller(IsoType::Network),
                DispatchAction::AutoInstallerNetwork,
            ),
            (Commands::ServeAnswers, DispatchAction::ServeAnswers),
            (Commands::Help, DispatchAction::Help),
            (Commands::Exit, DispatchAction::Exit),
        ];
        for (cmd, expected_action) in test_cases {
            let action = dispatch(cmd);
            assert_eq!(action, expected_action);
        }
    }
}
