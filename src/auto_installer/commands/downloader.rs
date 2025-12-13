use crate::auto_installer::commands::constants::Commands;
use crate::iso::downloader::download_latest_iso;

/// Downloads the Proxmox VE ISO to the specified destination path.
///
/// # Arguments
/// * `dest_path` — Optional destination path for the ISO file.
///   - If `None`, a default path is used.
///   - The default path is resolved by `Commands::default_download_path()`,
///     which checks `XDG_DATA_HOME` and falls back to `~/.local/share`.
///
/// # Returns
/// This function returns no value and reports download progress and status
/// via standard output.
///
/// # Notes
/// - This function will not overwrite an existing valid ISO file at the destination.
/// - Errors during download or verification are printed to standard error.
pub async fn download_pve_iso(dest_path: Option<String>) {
    let path = dest_path.unwrap_or_else(Commands::default_download_path);

    println!("Downloading Proxmox VE @ Latest -> {}", path);
    let result = download_latest_iso(&path, None).await;
    match result {
        Ok((_path, _sha256, downloaded)) => {
            if downloaded {
                println!("Download completed successfully.");
            }
        }
        Err(e) => eprintln!("Download failed: {}", e),
    }
}
