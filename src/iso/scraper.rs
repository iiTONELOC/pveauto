use crate::iso::constants::{ISO_URL_REGEX_PATTERN, PROXMOX_DL_PG_URL};
use oxdl::validator::{is_valid_sha256, is_valid_url};
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

/// Fetches the Proxmox VE download page HTML content.
///
/// # Returns
/// A string containing the HTML content of the download page.
/// # Errors
/// Returns an error if the HTTP request fails.
pub async fn fetch_dl_page() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(PROXMOX_DL_PG_URL).send().await?;
    let resp = resp.error_for_status()?;
    Ok(resp.text().await?)
}

/// Validates the scraped ISO URL and SHA256 checksum.
///
/// # Arguments
/// * `iso_url` - The ISO URL to validate.
/// * `sha256_checksum` - The SHA256 checksum to validate.
/// # Returns
/// Returns Ok(()) if both the ISO URL and SHA256 checksum are valid, otherwise returns an error.
/// # Errors
/// Returns an error if validation fails.
pub fn validate_return_data(iso_url: &str, sha256_checksum: &str) -> Result<(), Box<dyn Error>> {
    if !is_valid_url(iso_url) {
        return Err("Invalid ISO URL".into());
    }

    let iso_url_regex = Regex::new(ISO_URL_REGEX_PATTERN)?;
    if !iso_url_regex.is_match(iso_url) {
        return Err("ISO URL does not match expected pattern".into());
    }

    if !is_valid_sha256(sha256_checksum) {
        return Err("Invalid SHA256 checksum".into());
    }
    Ok(())
}

/// Scrapes the Proxmox VE download page to extract the latest ISO URL and its SHA256 checksum.
///
/// # Returns
/// A tuple containing the ISO URL and SHA256 checksum as strings.
/// # Errors
/// Returns an error if the scraping or data validation fails.
pub async fn get_latest_iso_info() -> Result<(String, String), Box<dyn Error>> {
    let html = fetch_dl_page().await?;
    let document = Html::parse_document(&html);

    let sel_latest = Selector::parse("ul.latest-downloads")?;
    let sel_li = Selector::parse("li")?;
    let sel_buttons = Selector::parse("div.download-entry-buttons")?;
    let sel_primary = Selector::parse("a.button-primary")?;
    let sel_info = Selector::parse("div.download-entry-info")?;
    let sel_dl = Selector::parse("dl")?;
    let sel_shasum = Selector::parse("div.download-entry-shasum")?;
    let sel_dd = Selector::parse("dd")?;
    let sel_code = Selector::parse("code")?;

    let latest = document
        .select(&sel_latest)
        .next()
        .ok_or("Latest downloads section not found")?;

    let second_li = latest
        .select(&sel_li)
        .nth(1)
        .ok_or("Second list item not found")?;

    let buttons = second_li
        .select(&sel_buttons)
        .next()
        .ok_or("Download buttons not found")?;

    let iso_url = buttons
        .select(&sel_primary)
        .next()
        .ok_or("ISO link not found")?
        .value()
        .attr("href")
        .ok_or("ISO link href not found")?
        .to_string();

    let info = second_li
        .select(&sel_info)
        .next()
        .ok_or("Download entry info not found")?;

    let sha256_checksum = info
        .select(&sel_dl)
        .next()
        .ok_or("DL element not found")?
        .select(&sel_shasum)
        .next()
        .ok_or("SHA sum element not found")?
        .select(&sel_dd)
        .next()
        .ok_or("DD element not found")?
        .select(&sel_code)
        .next()
        .ok_or("Code element not found")?
        .text()
        .next()
        .ok_or("Checksum text not found")?
        .trim()
        .to_string();

    validate_return_data(&iso_url, &sha256_checksum)?;
    Ok((iso_url, sha256_checksum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_iso_info() {
        let result = get_latest_iso_info().await;
        assert!(result.is_ok());
        let (iso_url, sha256_checksum) = result.unwrap();
        let iso_url_regex = Regex::new(ISO_URL_REGEX_PATTERN).unwrap();
        assert!(iso_url_regex.is_match(&iso_url));
        assert_eq!(sha256_checksum.len(), 64);
        println!("ISO URL: {}", iso_url);
        println!("SHA256 Checksum: {}", sha256_checksum);
    }
}
