# idevice_pair (appdb fork)

This repository is a **fork of [idevice_pair](https://github.com/jkcoxson/idevice_pair)** aimed at **[appdb](https://appdb.to)** users. It adds **automatic linking of the generated pairing file to your device on appdb**, so you can use appdb’s flows without manually uploading or associating the pairing file elsewhere.

Upstream is the cross-platform GUI for iOS device pairing and wireless debugging; this fork keeps that behavior and layers appdb-specific integration on top.

## Upstream

- **Original project:** [https://github.com/jkcoxson/idevice_pair](https://github.com/jkcoxson/idevice_pair)

## Why a fork?

Integration with appdb was proposed to upstream in [Pull Request #61](https://github.com/jkcoxson/idevice_pair/pull/61). The maintainers **declined to merge appdb support** into the main project. The reason was not explained in detail; the response on that PR was that they were not interested in appdb integration and that a fork was fine to maintain separately.

This fork exists so appdb users get a supported path that includes **pairing file handling tied to their device on appdb**, without depending on upstream to carry that integration.

## What you get (in addition to upstream features)

- Same core capabilities as upstream: USB device discovery, pairing file generation (RPPairing / Lockdown modes), validation, and installs into supported apps where applicable.
- **appdb:** pairing output is connected to your appdb device automatically (see in-app flow and [appdb](https://appdb.to) documentation for details).

Other app integrations that exist in this tree (SideStore, StikDebug, etc.) follow the same patterns as upstream unless noted in release notes.

## Prerequisites

- **macOS, Linux, or Windows** with **usbmuxd** (and on Windows, Apple’s USB stack as usual—e.g. iTunes from Apple).
- **iOS/iPadOS device** with a **passcode**, connected over **USB** when pairing.
- **Rust** (only if you build from source).

## Build from source

1. Clone **this** repository (use the clone URL shown on your Git host).

2. Build:

   ```bash
   cd idevice_pair
   cargo build --release
   ```

3. Run:

   ```bash
   cargo run --release
   ```

Prebuilt binaries for the **original** project are on [upstream releases](https://github.com/jkcoxson/idevice_pair/releases). This fork may publish its own releases separately—check this repository’s releases tab.

## Quick usage

1. Connect the device via USB and trust the computer if prompted.
2. Open the app and select your device.
3. Choose **RPPairing** (iOS 17.4+) or **Lockdown** (older), then **Generate** or **Load** as appropriate.
4. For **appdb**, complete the in-app steps so the pairing file is linked to your device on appdb.
5. For other targets, use **Install** under the listed app when shown.

For fuller pairing prerequisites (passcode, Developer Mode, etc.), see upstream’s README and [appdb](https://appdb.to) instructions.

## Troubleshooting

- **Device not seen:** USB, trust, and usbmuxd / iTunes (Windows) as in upstream docs.
- **Pairing or wireless issues:** same guidance as upstream (same network, port **62078**, firewall).

## License

This project inherits the **MIT License** from upstream. See [upstream](https://github.com/jkcoxson/idevice_pair) for the canonical license text.

## Acknowledgments

- **[idevice_pair](https://github.com/jkcoxson/idevice_pair)** — original application and design.
- **[idevice](https://crates.io/crates/idevice)** and **[egui](https://github.com/emilk/egui)** — same stack as upstream.
