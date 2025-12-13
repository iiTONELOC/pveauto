#[derive(Debug, PartialEq, Eq)]
pub enum DispatchAction {
    Download { dest_path: Option<String> },
    AutoInstallerOffline,
    AutoInstallerNetwork,
    ServeAnswers,
    Help,
    Exit,
}
