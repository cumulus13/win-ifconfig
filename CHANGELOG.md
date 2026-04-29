# Changelog

All notable changes to win-ifconfig will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.11/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.11] - 2026-04-28

### Added
- **Write/configure subcommands** (all require Administrator):
  - `ifconfig up <iface>` — bring interface UP via netsh
  - `ifconfig down <iface>` — bring interface DOWN via netsh
  - `ifconfig set <iface> ip <CIDR> [--gw GW]` — set static IPv4 address + gateway
  - `ifconfig set <iface> ip dhcp` — enable DHCP for address assignment
  - `ifconfig set <iface> add <CIDR>` — add a secondary IPv4 address
  - `ifconfig set <iface> del <IP>` — remove an IPv4 address
  - `ifconfig set <iface> mtu <N>` — set MTU (via `netsh interface ipv4 set subinterface`)
  - `ifconfig set <iface> metric <N>` — set routing metric
  - `ifconfig set <iface> dns <IP...>` — set one or more DNS servers
  - `ifconfig set <iface> dns dhcp` — reset DNS to automatic/DHCP
  - `ifconfig set <iface> mac <MAC>` — change MAC address via PowerShell `Set-NetAdapter`
  - `ifconfig set <iface> flush` — flush all IPs and reset to DHCP
- New `control.rs` module with all netsh/PowerShell wrappers
- CIDR notation parsing and validation (`192.168.1.50/24`)
- Error messages with `💡 Tip: Run as Administrator` hint when access is denied
- Colorful operation feedback (green ✅, yellow 🔧, cyan 🔄)

## [1.0.5] - 2026-04-27

### Added
- Initial release
- Linux-compatible `ifconfig` output format for Windows
- Colorful ANSI output with emoji adapter type indicators
- Support for all Windows adapter types: Ethernet, Wi-Fi, Loopback, Tunnel, PPP, VPN
- IPv4 and IPv6 address display with scope labels (global, link, host)
- Per-interface RX/TX statistics: packets, bytes, errors, drops, unicast/multicast/broadcast
- Interface flags: UP, BROADCAST, RUNNING, MULTICAST, LOOPBACK, DHCP, etc.
- Interface metric and routing priority display (`--metrics`)
- DHCP lease info: server, lease obtained, lease expiry (`--dhcp`)
- DNS server list per interface (`--dns`)
- WINS server display
- MAC address display with txqueuelen
- Interface speed (Mbps/Gbps) display
- MTU display
- Filter by interface name or index
- `--all` flag to show disconnected/DOWN interfaces
- `--brief` compact one-liner mode
- `--json` machine-readable JSON output
- `--watch N` auto-refresh watch mode
- `--summary` totals at the end
- `--verbose` show all extended info in one pass
- `--no-color` for piping to files/scripts
- GitHub Actions CI workflow (lint, build, test on Windows)
- GitHub Actions release workflow with multi-arch builds (x64, x86, ARM64)
- Automated crates.io publishing on release tags

[Unreleased]: https://github.com/cumulus13/win-ifconfig/compare/v1.0.11...HEAD
[1.0.5]: https://github.com/cumulus13/win-ifconfig/releases/tag/v1.0.11
