use crate::auto_installer::commands::constants::USAGE;
use crate::auto_installer::commands::downloader::download_pve_iso;
use crate::auto_installer::dispatcher::actions::DispatchAction;
use crate::iso::scraper::IsoError;
use std::fmt;

#[derive(Debug)]
pub enum ExecuteError {
    Iso(IsoError),
    NotImplemented(&'static str),
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iso(e) => write!(f, "{e}"),
            Self::NotImplemented(command) => write!(f, "`{command}` is not implemented yet"),
        }
    }
}

impl std::error::Error for ExecuteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Iso(e) => Some(e),
            Self::NotImplemented(_) => None,
        }
    }
}

impl From<IsoError> for ExecuteError {
    fn from(e: IsoError) -> Self {
        Self::Iso(e)
    }
}

pub async fn execute(action: DispatchAction) -> Result<(), ExecuteError> {
    match action {
        DispatchAction::Download { dest_path } => Ok(download_pve_iso(dest_path).await?),
        DispatchAction::AutoInstallerOffline => {
            Err(ExecuteError::NotImplemented("offline-installer"))
        }
        DispatchAction::AutoInstallerNetwork => {
            Err(ExecuteError::NotImplemented("network-installer"))
        }
        DispatchAction::ServeAnswers => Err(ExecuteError::NotImplemented("serve-answers")),
        DispatchAction::Help => {
            println!("{USAGE}");
            Ok(())
        }
        DispatchAction::Exit => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unbuilt_commands_report_not_implemented() {
        for (action, name) in [
            (DispatchAction::AutoInstallerOffline, "offline-installer"),
            (DispatchAction::AutoInstallerNetwork, "network-installer"),
            (DispatchAction::ServeAnswers, "serve-answers"),
        ] {
            let err = execute(action).await.expect_err("not implemented");
            assert!(
                matches!(err, ExecuteError::NotImplemented(n) if n == name),
                "{err}"
            );
        }
    }
}
