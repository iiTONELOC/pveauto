# PVE Auto | Proxmox Virtual Environment Auto Installer

![PVEAUTO](https://raw.githubusercontent.com/iiTONELOC/pveauto/main/assets/images/pveauto.jpg)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Verification](#verification)
- [Exit Codes](#exit-codes)
- [License](#license)
- [Contact](#contact)
- [Contributing](#contributing)

## Overview

PVE Auto is a Rust library and command-line tool that downloads the latest Proxmox VE ISO and verifies it against the checksum Proxmox publishes. It also carries a typed, validated model of the `[global]` section of a Proxmox answer file for unattended installs.

> [!Caution]
> This project is in early development. The commands marked planned below exist in the CLI but return `not implemented yet`.

## Features

### Implemented

- Find the latest Proxmox VE ISO and its SHA-256 on the Proxmox download page.
- Download it with [0xDL](https://crates.io/crates/oxdl): atomic placement, typed errors, no OpenSSL dependency.
- Verify the download, or an existing file at the destination, against the published checksum. A valid existing file is kept. An invalid one is replaced atomically.
- Typed `[global]` answer-file section with validation of keyboard, country, timezone, FQDN, mailto, hashed root password, SSH keys, and reboot mode.
- Non-zero exit codes on every failure, so automation can detect them.

### Planned

- Embed an answer file into the ISO for offline unattended installs.
- Serve answer files over the local network for the network installer.

## Installation

### As a Library

```toml
[dependencies]
pveauto = "0.2.0"
```

```bash
cargo add pveauto
```

### As a Command-Line Tool

```bash
cargo install pveauto
```

Requires Rust 1.85 or newer. Linux is supported. macOS is untested.

## Usage

### Command Line

```text
Usage: pveauto <command> [options]

Commands:
  download [path]     Download Proxmox VE ISO to [path], defaults to:
                        $XDG_DATA_HOME/pve-auto/proxmox-ve-latest.iso
                        or ~/.local/share/pve-auto/proxmox-ve-latest.iso
  offline-installer   Create unattended ISO (offline; requires MGMT MAC)   [planned]
  network-installer   Create unattended ISO (network; DHCP required)       [planned]
  serve-answers       Start HTTP server for network installer              [planned]
  exit                Exit program
```

```bash
pveauto download /optional/path/to/save/proxmox-ve-latest.iso
```

> [!Note]
> If no path is provided, the ISO is saved to `$XDG_DATA_HOME/pve-auto/proxmox-ve-latest.iso`, or `~/.local/share/pve-auto/proxmox-ve-latest.iso` when `XDG_DATA_HOME` is unset.

![PVEAUTO CLI](https://raw.githubusercontent.com/iiTONELOC/pveauto/main/assets/images/pveauto-cli.png)

### Library

```rust
use pveauto::iso::downloader::download_latest_iso;

#[tokio::main]
async fn main() -> Result<(), pveauto::iso::scraper::IsoError> {
    let result = download_latest_iso("proxmox-ve-latest.iso", None).await?;
    println!("{} sha256={} downloaded={}", result.path, result.sha256, result.downloaded);
    Ok(())
}
```

`download_latest_iso` returns `IsoDownload { path, sha256, downloaded }`. `downloaded` is `false` when an existing file already matched the checksum. Errors are `IsoError`: `Fetch`, `Layout`, `InvalidIsoUrl`, `InvalidChecksum`, `Download`, or `Verify`, each carrying its cause.

## Verification

The ISO URL and its SHA-256 are read from the same Proxmox download page over HTTPS. The checksum is then compared against the downloaded bytes before they are placed at the destination. That proves the file arrived intact and matches what Proxmox published on the page. It does not defend against a compromised download page, since the page is the source of both values.

The scraper only accepts ISO URLs under `enterprise.proxmox.com` and 64-character hex checksums. A page layout change produces a `Layout` error naming the missing element rather than a wrong download.

## Exit Codes

| Code | Meaning |
| --- | --- |
| 0 | Success |
| 1 | The command ran and failed, with the reason on stderr |
| 2 | Unknown or missing command, with usage on stderr |

## Testing

```bash
cargo test
```

The suite runs offline. The scraper is tested against a fixture of the download page. Live tests against proxmox.com are opt-in:

```bash
cargo test --features iso-scraper-tests
cargo test --features iso-download-tests
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE.txt) for details.

## Contact

Created by Anthony Tropeano

- [GitHub](https://github.com/iitoneloc)
- [Website](https://www.atropeano.com)

## Contributing

Contributions are welcome. Please feel free to submit issues and pull requests.
