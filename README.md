# 🌐 win-ifconfig

> **Linux-style `ifconfig` for Windows** — colorful, feature-rich, production-ready

[![Build & Release](https://github.com/cumulus13/win-ifconfig/actions/workflows/release.yml/badge.svg)](https://github.com/cumulus13/win-ifconfig/actions/workflows/release.yml)
[![CI](https://github.com/cumulus13/win-ifconfig/actions/workflows/ci.yml/badge.svg)](https://github.com/cumulus13/win-ifconfig/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ifconfig.svg)](https://crates.io/crates/ifconfig)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform: Windows](https://img.shields.io/badge/platform-Windows-blue?logo=windows)](https://github.com/cumulus13/win-ifconfig/releases)

```
╔══════════════════════════════════════════════════════════════════════╗
║  🌐  win-ifconfig v1.0.11  ─  Windows Network Interface Information  ║
╚══════════════════════════════════════════════════════════════════════╝
  🖥️  Hostname: WORKSTATION-01   🕐 2024-01-15 14:32:07 WIB

🔌 Ethernet: <UP,BROADCAST,RUNNING,MULTICAST,DHCP> UP mtu 1500 metric 25
        link/ether  Intel(R) Ethernet Connection I219-V
        ether    A4:BB:6D:23:91:F0 txqueuelen 1000
        speed    1 Gbps
        inet     192.168.1.100  netmask 255.255.255.0  broadcast 192.168.1.255  scope global
        inet6    fe80::1a2b:3c4d:5e6f:7890/64  scope link
        gateway  192.168.1.1

        📥 RX packets:7654321 errors:3 dropped:12 overruns:0 frame:0
           RX bytes:9876543210  (9.20 GiB)
        📤 TX packets:1234567 errors:0 dropped:1 overruns:0 carrier:0
           TX bytes:1234567890  (1.15 GiB)
               collisions:0
```

---

## ✨ Features

| Feature | Details |
|---------|---------|
| 🎨 **Colorful output** | ANSI colors, emoji indicators for adapter type and status |
| 🐧 **Linux-compatible** | Same field layout as `ifconfig` on Linux — familiar for sysadmins |
| 📊 **Extended metrics** | Interface metric, routing priority, speed, duplex |
| 📶 **All adapter types** | Ethernet, Wi-Fi, Loopback, VPN, Tunnel, PPP, Bridge |
| 🔢 **Full statistics** | RX/TX packets, bytes, errors, drops, unicast, multicast, broadcast |
| 🏷️ **DHCP info** | Server IP, lease obtained & expiry timestamps |
| 🔍 **DNS per adapter** | All DNS servers assigned to each interface |
| 🌐 **IPv6 support** | Global, link-local, site-local with scope labels |
| 📄 **JSON output** | Machine-readable JSON for scripting and automation |
| 🔄 **Watch mode** | Auto-refresh with `--watch N` |
| ⚡ **Zero runtime** | Single `.exe`, no .NET/runtime required |
| 🪟 **Native WinAPI** | Uses `GetAdaptersAddresses` + `GetIfEntry2` directly |

---

## 📥 Installation

### Option 1 — Download prebuilt binary

Download from [GitHub Releases](https://github.com/cumulus13/win-ifconfig/releases):

| Platform | File |
|----------|------|
| Windows x64 (64-bit) | `win-ifconfig-x86_64-windows.exe` |
| Windows x86 (32-bit) | `win-ifconfig-i686-windows.exe` |
| Windows ARM64 | `win-ifconfig-aarch64-windows.exe` |

Rename to `ifconfig.exe` and place it in any folder on your `PATH` (e.g., `C:\Windows\System32\` or `%USERPROFILE%\bin\`).

### Option 2 — Install via cargo

```powershell
cargo install win-ifconfig
```

> Requires Rust toolchain. Get it at [rustup.rs](https://rustup.rs).

### Option 3 — Build from source

```powershell
git clone https://github.com/cumulus13/win-ifconfig.git
cd win-ifconfig
cargo build --release
# Binary at: target\release\ifconfig.exe
```

---

## 🚀 Usage

### Read (display) — no privileges needed

```
USAGE:
    ifconfig [OPTIONS] [INTERFACE]

ARGUMENTS:
    [INTERFACE]   Interface name or index (shows all active if omitted)

OPTIONS:
    -a, --all         Show all interfaces (including DOWN and loopback)
    -s, --stats       Show extended per-direction statistics
    -m, --metrics     Show routing metric and gateway details
    -d, --dns         Show DNS server list per interface
        --dhcp        Show DHCP lease information
    -v, --verbose     Show all extended info (stats + metrics + dns + dhcp)
    -b, --brief       Compact one-line-per-adapter output
    -j, --json        Output in JSON format
    -w, --watch <N>   Watch/refresh mode, update every N seconds
        --summary     Show totals summary at the end
        --no-color    Disable colored output
    -h, --help        Print help
    -V, --version     Print version
```

### Write (configure) — ⚠️ requires Administrator

```
SUBCOMMANDS:
    up   <IFACE>                        Bring interface UP
    down <IFACE>                        Bring interface DOWN
    set  <IFACE> ip <CIDR> [--gw GW]   Set static IPv4 address
    set  <IFACE> ip dhcp                Enable DHCP for address
    set  <IFACE> add <CIDR>             Add secondary IPv4 address
    set  <IFACE> del <IP>               Remove an IPv4 address
    set  <IFACE> mtu <N>                Set MTU in bytes
    set  <IFACE> metric <N>             Set routing metric
    set  <IFACE> dns <IP> [IP...]       Set DNS servers
    set  <IFACE> dns dhcp               Use automatic (DHCP) DNS
    set  <IFACE> mac <MAC>              Change MAC address
    set  <IFACE> flush                  Flush IPs, reset to DHCP
```

### Read examples

```powershell
ifconfig                          # Active interfaces
ifconfig -a                       # All interfaces including DOWN
ifconfig "Wi-Fi"                  # Specific interface
ifconfig -v                       # Full verbose (stats+metrics+dns+dhcp)
ifconfig --stats                  # Packet/byte/error stats
ifconfig --metrics                # Routing metrics
ifconfig --dns                    # DNS per interface
ifconfig --dhcp                   # DHCP lease details
ifconfig -b                       # Compact one-liner per interface
ifconfig --json                   # Machine-readable JSON
ifconfig --watch 2                # Auto-refresh every 2 seconds
ifconfig "Ethernet" --watch 1 -s  # Watch specific interface with stats
ifconfig -a --summary             # Show with totals
ifconfig --no-color > net.txt     # Plain text for log files
```

### Write examples (run as Administrator)

```powershell
# Bring interfaces up/down
ifconfig up Ethernet
ifconfig down "Wi-Fi"

# Set static IP with gateway
ifconfig set "Ethernet" ip 192.168.1.100/24 --gw 192.168.1.1

# Enable DHCP
ifconfig set "Wi-Fi" ip dhcp

# Add a secondary IP address
ifconfig set "Ethernet" add 10.0.0.50/24

# Remove an IP address
ifconfig set "Ethernet" del 10.0.0.50

# Set MTU (e.g. jumbo frames)
ifconfig set "Ethernet" mtu 9000

# Set routing metric (lower = higher priority)
ifconfig set "Wi-Fi" metric 10
ifconfig set "Ethernet" metric 5

# Set custom DNS servers
ifconfig set "Wi-Fi" dns 8.8.8.8 8.8.4.4
ifconfig set "Ethernet" dns 1.1.1.1 1.0.0.1 9.9.9.9

# Reset DNS to DHCP/automatic
ifconfig set "Wi-Fi" dns dhcp

# Change MAC address (adapter must support it)
ifconfig set "Ethernet" mac AA:BB:CC:DD:EE:FF

# Flush all IPs and reset to DHCP
ifconfig set "Ethernet" flush
```

---

## 📊 Output Fields Reference

### Header line
```
🔌 Ethernet: <UP,BROADCAST,RUNNING,MULTICAST,DHCP> UP mtu 1500 metric 25
```
| Field | Meaning |
|-------|---------|
| Emoji | Adapter type (🔌 Ethernet, 📶 Wi-Fi, 🔁 Loopback, 🔒 VPN, 🚇 Tunnel) |
| `<FLAGS>` | Adapter flags (UP, BROADCAST, RUNNING, MULTICAST, DHCP, LOOPBACK, POINTOPOINT…) |
| Status | `UP` (green) or `DOWN` (red) |
| `mtu` | Maximum Transmission Unit in bytes |
| `metric` | Windows routing metric — lower = higher priority |

### Address lines
```
inet   192.168.1.100  netmask 255.255.255.0  broadcast 192.168.1.255  scope global
inet6  fe80::1a2b:3c4d:5e6f:7890/64  scope link
ether  A4:BB:6D:23:91:F0
```

### Statistics lines
```
📥 RX packets:7654321 errors:3 dropped:12 overruns:0 frame:0
   RX bytes:9876543210  (9.20 GiB)
📤 TX packets:1234567 errors:0 dropped:1 overruns:0 carrier:0
   TX bytes:1234567890  (1.15 GiB)
```

### Extended (--verbose / --stats)
```
       unicast:7000000 multicast:50000 broadcast:604321
```

### Metrics (--metrics / -v)
```
📊 METRIC: 25
⚡ SPEED:  1 Gbps
📦 MTU:    1500
🔢 INDEX:  2
```

### DHCP (--dhcp / -v)
```
🏷️  DHCP: enabled
    Server:          192.168.1.1
    Lease obtained:  2024-01-15 08:00:00
    Lease expires:   2024-01-16 08:00:00
```

### DNS (--dns / -v)
```
🔍 DNS servers:
     8.8.8.8
     8.8.4.4
```

---

## 🔧 Technical Details

**Language:** Rust 2021 edition  
**Windows APIs used:**
- `GetAdaptersAddresses` — adapter enumeration with full address/DNS/gateway/WINS info
- `GetIfEntry2` — per-interface statistics (RX/TX bytes, packets, errors, drops, unicast/multicast/broadcast)
- `MIB_IF_ROW2` — speed, MTU, operational status

**Dependencies:**
| Crate | Purpose |
|-------|---------|
| `windows` | Windows API bindings (IpHelper, WinSock) |
| `clap` | CLI argument parsing |
| `colored` | ANSI terminal colors |
| `serde` + `serde_json` | JSON serialization |
| `chrono` | Date/time formatting for DHCP leases |
| `hostname` | System hostname display |
| `crossterm` | Terminal control (watch mode) |

**Binary size:** ~1.5 MB stripped (release + LTO + strip)  
**Runtime requirements:** None (statically linked, no .NET/MSVC runtime needed)

---

## 🏗️ Building

### Prerequisites

- Rust 1.75+ (`rustup install stable`)
- Windows SDK (MSVC) or MinGW-w64

### Build commands

```powershell
# Debug build
cargo build

# Release build (optimized, ~1.5MB)
cargo build --release

# Run tests
cargo test

# Check without building
cargo check
```

### Cross-compile from Linux

```bash
# Install MinGW toolchain
sudo apt install gcc-mingw-w64-x86-64

# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

---

## 🤝 Contributing

Pull requests are welcome! Please:

1. Fork the repo and create a branch: `git checkout -b feature/my-feature`
2. Make changes and run: `cargo fmt && cargo clippy && cargo test`
3. Commit with descriptive messages
4. Push and open a PR

### Areas for contribution
- [ ] IPv6 gateway detection
- [ ] Wi-Fi signal strength (RSSI) via WlanQueryInterface
- [ ] VPN detection heuristics
- [ ] Vendor lookup from MAC OUI database
- [ ] Windows Firewall status per interface
- [ ] Network adapter power management state
- [ ] LLDP neighbor info

---

## 📜 License

MIT License — see [LICENSE](LICENSE) for details.

---

## 👤 Author
        
[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

---

## 🙏 Acknowledgements

- Inspired by `net-tools` `ifconfig` on Linux
- Windows IP Helper API documentation by Microsoft
- The Rust `windows-rs` crate by Microsoft
