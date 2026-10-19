use crate::iso::constants::{ISO_URL_REGEX_PATTERN, PROX_DL_PG_URL};
use oxdl::validator::{is_valid_sha256, is_valid_url};
use regex::Regex;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::fmt;

const SEL_LATEST: &str = "ul.latest-downloads";
const SEL_ENTRY: &str = "li";
const SEL_BUTTONS: &str = "div.download-entry-buttons";
const SEL_PRIMARY: &str = "a.button-primary";
const SEL_INFO: &str = "div.download-entry-info";
const SEL_SHASUM: &str = "div.download-entry-shasum";
const SEL_CODE: &str = "code";
const ISO_ENTRY: &str = "li with a Proxmox VE ISO download link";

#[derive(Debug)]
pub enum IsoError {
    Fetch(reqwest::Error),
    Layout(&'static str),
    InvalidIsoUrl(String),
    InvalidChecksum(String),
    Download(oxdl::DownloadError),
    Verify(std::io::Error),
}

impl fmt::Display for IsoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch(e) => write!(f, "could not fetch the Proxmox download page: {e}"),
            Self::Layout(selector) => {
                write!(f, "download page layout changed: `{selector}` not found")
            }
            Self::InvalidIsoUrl(url) => write!(f, "scraped ISO URL is not a Proxmox ISO: {url}"),
            Self::InvalidChecksum(sum) => write!(f, "scraped checksum is not SHA-256: {sum}"),
            Self::Download(e) => write!(f, "download failed: {e}"),
            Self::Verify(e) => write!(f, "could not read the existing ISO: {e}"),
        }
    }
}

impl std::error::Error for IsoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fetch(e) => Some(e),
            Self::Download(e) => Some(e),
            Self::Verify(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for IsoError {
    fn from(e: reqwest::Error) -> Self {
        Self::Fetch(e)
    }
}

impl From<oxdl::DownloadError> for IsoError {
    fn from(e: oxdl::DownloadError) -> Self {
        Self::Download(e)
    }
}

/// Fetches the Proxmox VE download page HTML content.
pub async fn fetch_dl_page() -> Result<String, IsoError> {
    let resp = Client::new().get(PROX_DL_PG_URL).send().await?;
    Ok(resp.error_for_status()?.text().await?)
}

/// Validates the scraped ISO URL and SHA256 checksum.
pub fn validate_return_data(iso_url: &str, sha256_checksum: &str) -> Result<(), IsoError> {
    let pattern = Regex::new(ISO_URL_REGEX_PATTERN)
        .map_err(|_| IsoError::InvalidIsoUrl(iso_url.to_string()))?;
    if !is_valid_url(iso_url) || !pattern.is_match(iso_url) {
        return Err(IsoError::InvalidIsoUrl(iso_url.to_string()));
    }
    if !is_valid_sha256(sha256_checksum) {
        return Err(IsoError::InvalidChecksum(sha256_checksum.to_string()));
    }
    Ok(())
}

fn select<'a>(scope: ElementRef<'a>, css: &'static str) -> Result<ElementRef<'a>, IsoError> {
    let selector = Selector::parse(css).map_err(|_| IsoError::Layout(css))?;
    scope.select(&selector).next().ok_or(IsoError::Layout(css))
}

fn primary_href(entry: ElementRef<'_>) -> Option<String> {
    let link = select(select(entry, SEL_BUTTONS).ok()?, SEL_PRIMARY).ok()?;
    link.value().attr("href").map(str::to_string)
}

/// Extracts the newest ISO URL and its SHA256 checksum from download page HTML.
/// The newest ISO is the first list entry whose primary link is a Proxmox VE ISO.
pub fn parse_latest_iso_info(html: &str) -> Result<(String, String), IsoError> {
    let document = Html::parse_document(html);
    let latest = select(document.root_element(), SEL_LATEST)?;
    let entries = Selector::parse(SEL_ENTRY).map_err(|_| IsoError::Layout(SEL_ENTRY))?;
    let pattern = Regex::new(ISO_URL_REGEX_PATTERN).map_err(|_| IsoError::Layout(ISO_ENTRY))?;
    let (entry, iso_url) = latest
        .select(&entries)
        .filter_map(|entry| primary_href(entry).map(|href| (entry, href)))
        .find(|(_, href)| pattern.is_match(href))
        .ok_or(IsoError::Layout(ISO_ENTRY))?;

    let sha256_checksum = select(select(select(entry, SEL_INFO)?, SEL_SHASUM)?, SEL_CODE)?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    validate_return_data(&iso_url, &sha256_checksum)?;
    Ok((iso_url, sha256_checksum))
}

/// Scrapes the Proxmox VE download page for the latest ISO URL and its SHA256 checksum.
pub async fn get_latest_iso_info() -> Result<(String, String), IsoError> {
    parse_latest_iso_info(&fetch_dl_page().await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ISO_URL: &str = "https://enterprise.proxmox.com/iso/proxmox-ve_9.1-1.iso";
    const ISO_SHA256: &str = "6d8f5afc78c0c66812d7272cde7c8b98be7eb54401ceb045400db05eb5ae6d22";

    const OLDER_ISO_URL: &str = "https://enterprise.proxmox.com/iso/proxmox-ve_8.4-1.iso";
    const OLDER_SHA256: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn entry(url: &str, sum: Option<&str>) -> String {
        let shasum = sum.map_or(String::new(), |sum| {
            format!(
                r#"<div class="download-entry-shasum"><dt>SHA256SUM</dt><dd><code> {sum} </code></dd></div>"#
            )
        });
        format!(
            r#"<li><div class="download-entry box-light">
  <div class="download-entry-info"><dl><div><dt>Version</dt><dd>x</dd></div>{shasum}</dl></div>
  <div class="download-entry-buttons"><a class="button button-primary" href="{url}">Download</a></div>
</div></li>"#
        )
    }

    fn page(url: &str, sum: &str) -> String {
        format!(
            r#"<html><body><ul class="mod-list latest-downloads">
{}
{}
{}
</ul></body></html>"#,
            entry("/images/download/pve/agreements/Agreement.pdf", None),
            entry(url, Some(sum)),
            entry(OLDER_ISO_URL, Some(OLDER_SHA256)),
        )
    }

    #[test]
    fn picks_the_first_iso_entry_and_skips_other_downloads() {
        let (url, sum) = parse_latest_iso_info(&page(ISO_URL, ISO_SHA256)).expect("parse");
        assert_eq!(url, ISO_URL);
        assert_eq!(sum, ISO_SHA256);
    }

    #[test]
    fn missing_section_is_a_layout_error() {
        let err = parse_latest_iso_info("<html><body></body></html>").expect_err("layout");
        assert!(matches!(err, IsoError::Layout(SEL_LATEST)), "{err}");
    }

    #[test]
    fn page_without_an_iso_entry_is_a_layout_error() {
        let html = format!(
            r#"<ul class="latest-downloads">{}</ul>"#,
            entry("/images/download/pve/agreements/Agreement.pdf", None)
        );
        let err = parse_latest_iso_info(&html).expect_err("no iso");
        assert!(matches!(err, IsoError::Layout(ISO_ENTRY)), "{err}");
    }

    #[test]
    fn wrong_host_is_rejected() {
        let err = validate_return_data("https://example.com/proxmox-ve_9.1-1.iso", ISO_SHA256)
            .expect_err("url");
        assert!(matches!(err, IsoError::InvalidIsoUrl(_)), "{err}");
    }

    #[test]
    fn short_checksum_is_rejected() {
        let err = parse_latest_iso_info(&page(ISO_URL, &ISO_SHA256[..40])).expect_err("sum");
        assert!(matches!(err, IsoError::InvalidChecksum(_)), "{err}");
    }
}

#[cfg(all(test, feature = "iso-scraper-tests"))]
mod live_tests {
    use super::*;

    #[tokio::test]
    async fn live_page_parses() {
        let (iso_url, sha256_checksum) = get_latest_iso_info().await.expect("scrape");
        assert!(iso_url.ends_with(".iso"));
        assert_eq!(sha256_checksum.len(), 64);
    }
}
