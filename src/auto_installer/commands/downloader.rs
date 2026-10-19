use crate::auto_installer::commands::constants::Commands;
use crate::iso::downloader::download_latest_iso;
use crate::iso::scraper::IsoError;

/// Downloads the Proxmox VE ISO to `dest_path`, or to the default path when `None`.
///
/// Prints the destination and the outcome to standard output. An existing file that
/// matches the published checksum is kept. Errors are returned to the caller.
pub async fn download_pve_iso(dest_path: Option<String>) -> Result<(), IsoError> {
    let path = dest_path.unwrap_or_else(Commands::default_download_path);
    println!("Downloading Proxmox VE @ Latest -> {path}");
    let result = download_latest_iso(&path, None).await?;
    if result.downloaded {
        println!("Downloaded and verified: {}", result.path);
    } else {
        println!(
            "Existing file matches the published checksum: {}",
            result.path
        );
    }
    Ok(())
}
