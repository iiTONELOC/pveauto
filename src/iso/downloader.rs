use crate::iso::scraper::{IsoError, get_latest_iso_info};
use oxdl::{UpdateFn, download_with_updates, validator::verify_file_sha256};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct IsoDownload {
    pub path: String,
    pub sha256: String,
    pub downloaded: bool,
}

/// Downloads the latest Proxmox VE ISO to `to_file_path` unless a file that already
/// matches the published checksum is there. A new download replaces the file atomically.
pub async fn download_latest_iso(
    to_file_path: &str,
    with_custom_updater: Option<UpdateFn>,
) -> Result<IsoDownload, IsoError> {
    let (iso_url, sha256) = get_latest_iso_info().await?;

    if Path::new(to_file_path).exists()
        && verify_file_sha256(to_file_path, &sha256)
            .await
            .map_err(IsoError::Verify)?
    {
        return Ok(IsoDownload {
            path: to_file_path.to_string(),
            sha256,
            downloaded: false,
        });
    }

    download_with_updates(&iso_url, to_file_path, with_custom_updater, Some(&sha256)).await?;
    Ok(IsoDownload {
        path: to_file_path.to_string(),
        sha256,
        downloaded: true,
    })
}

#[cfg(all(test, feature = "iso-download-tests"))]
mod live_tests {
    use super::*;

    #[tokio::test]
    async fn live_download_verifies_and_then_skips() {
        let dir = tempfile::tempdir().expect("tempdir");
        let dest = dir.path().join("proxmox-ve-latest.iso");
        let path = dest.to_str().expect("utf8");
        let first = download_latest_iso(path, None).await.expect("download");
        assert!(first.downloaded);
        let second = download_latest_iso(path, None).await.expect("verify");
        assert!(!second.downloaded);
        assert_eq!(first.sha256, second.sha256);
    }
}
