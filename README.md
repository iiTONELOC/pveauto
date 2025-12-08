# PVE Auto | Proxmox Virtual Environment Auto Installer

![PVEAUTO](./pveauto.jpg)

## Coming Soon

This repository is a Rust-based library and command-line tool designed to download and Verify Proxmox Virtual Environment ISO images automatically.

`PVE Auto` can also provide a means for [unattended installations of Proxmox VE](https://pve.proxmox.com/wiki/Unattended_installation_of_Proxmox) by embedding the necessary configuration files into the ISO image (if end-host configuration is known) or by serving them over the local network during the auto-installation process.

## Features

- Download the latest Proxmox VE ISO image automatically.
- Verify the integrity of the downloaded ISO using checksums.
- Embed unattended installation configuration files into the ISO image.
- Serve configuration files over the local network for auto-installation.
- Command-line interface for easy usage.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Contact

Created by Anthony Tropeano

- [GitHub](https://github.com/iitoneloc)
- [Website](https://www.atropeano.com)
