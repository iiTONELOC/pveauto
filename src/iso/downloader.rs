use crate::iso::scraper::get_latest_iso_info;
use oxdl::{download_with_updates, validator::verify_file_sha256};

pub async fn download_latest_iso(
    to_file_path: &str,
    with_custom_updater: Option<Box<dyn Fn(f32) + Send + Sync>>,
) -> Result<(String, String, bool), Box<dyn std::error::Error>> {
    let (iso_url, sha256_checksum) = get_latest_iso_info().await?;

    // check if the file already exists has size and valid checksum
    if std::path::Path::new(to_file_path).exists() {
        println!("Existing file found. Verifying checksum...");
        let is_valid = verify_file_sha256(to_file_path, &sha256_checksum).await?;
        if is_valid {
            println!("Checksum valid. Skipping download.");
            return Ok((to_file_path.to_string(), sha256_checksum, false));
        } else {
            println!("Checksum mismatch. Re-downloading...");
        }
    } else {
        println!("No existing file found. Proceeding to download...");
    }

    let res: Result<(), oxdl::DownloadError> = download_with_updates(
        &iso_url,
        to_file_path,
        with_custom_updater,
        Some(&sha256_checksum),
    )
    .await;
    match res {
        Ok(_) => Ok((to_file_path.to_string(), sha256_checksum, true)),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(all(test, feature = "iso-download-tests"))]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_download_latest_iso() {
        let to_file_path = "test_proxmox_iso.iso";
        let result = download_latest_iso(to_file_path, None).await;
        let expected = get_latest_iso_info().await;
        assert!(expected.is_ok());

        assert!(result.is_ok());
        let (downloaded_path, sha256, downloaded) = result.unwrap();
        let (_, expected_sha256) = expected.unwrap();
        assert_eq!(downloaded_path, to_file_path);
        assert!(!sha256.is_empty());
        assert!(sha256.len() == 64);
        assert!(sha256 == expected_sha256);
        assert!(downloaded);
        assert!(Path::new(to_file_path).exists());

        // Clean up
        std::fs::remove_file(to_file_path).unwrap();
    }
}
