use crate::iso::scraper::get_latest_iso_info;
use oxdl::download_with_updates;

pub async fn download_latest_iso(
    to_file_path: &str,
    with_custom_updater: Option<Box<dyn Fn(f32) + Send + Sync>>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let (iso_url, sha256_checksum) = get_latest_iso_info().await?;
    let res: Result<(), oxdl::DownloadError> = download_with_updates(
        &iso_url,
        to_file_path,
        with_custom_updater,
        Some(&sha256_checksum),
    )
    .await;
    match res {
        Ok(_) => Ok((to_file_path.to_string(), sha256_checksum)),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(feature = "test-iso-download")]
#[cfg(test)]
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
        let (downloaded_path, sha256) = result.unwrap();
        let (_, expected_sha256) = expected.unwrap();
        assert_eq!(downloaded_path, to_file_path);
        assert!(!sha256.is_empty());
        assert!(sha256.len() == 64);
        assert!(sha256 == expected_sha256);
        assert!(Path::new(to_file_path).exists());

        // Clean up
        std::fs::remove_file(to_file_path).unwrap();
    }
}
