use crate::auto_installer::commands::downloader::download_pve_iso;
use crate::auto_installer::dispatcher::actions::DispatchAction;

pub async fn execute(action: DispatchAction) {
    match action {
        DispatchAction::Download { dest_path } => {
            download_pve_iso(dest_path).await;
        }
        DispatchAction::AutoInstallerOffline => {
            println!("Selected: offline installer");
        }
        DispatchAction::AutoInstallerNetwork => {
            println!("Selected: network installer");
        }
        DispatchAction::ServeAnswers => {
            println!("Selected: serve answers");
        }
        DispatchAction::Help => {
            println!("Selected: help");
        }
        DispatchAction::Exit => {
            println!("Selected: exit");
        }
    }
}
