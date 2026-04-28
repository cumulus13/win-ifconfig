// types.rs — Core data structures for win-ifconfig
// Author: Hadi Cahyadi <cumulus13@gmail.com>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    pub index: u32,
    pub name: String,
    pub friendly_name: String,
    pub description: String,
    pub adapter_type: AdapterType,
    pub status: AdapterStatus,
    pub mac_address: Option<String>,
    pub ipv4_addresses: Vec<IpAddress>,
    pub ipv6_addresses: Vec<IpAddress>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_enabled: bool,
    pub dhcp_server: Option<String>,
    pub dhcp_lease_obtained: Option<String>,
    pub dhcp_lease_expires: Option<String>,
    pub mtu: u32,
    pub speed: Option<u64>, // bps
    pub metric: u32,
    pub stats: Option<AdapterStats>,
    pub flags: AdapterFlags,
    pub wins_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAddress {
    pub address: String,
    pub prefix_len: u8,
    pub netmask: Option<String>,
    pub broadcast: Option<String>,
    pub scope: IpScope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpScope {
    Global,
    LinkLocal,
    Loopback,
    SiteLocal,
    Unknown,
}

impl std::fmt::Display for IpScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpScope::Global => write!(f, "global"),
            IpScope::LinkLocal => write!(f, "link"),
            IpScope::Loopback => write!(f, "host"),
            IpScope::SiteLocal => write!(f, "site"),
            IpScope::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdapterType {
    Ethernet,
    WiFi,
    Loopback,
    Tunnel,
    PPP,
    Bridge,
    VPN,
    Other(String),
}

impl std::fmt::Display for AdapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterType::Ethernet => write!(f, "Ethernet"),
            AdapterType::WiFi => write!(f, "IEEE 802.11 (Wi-Fi)"),
            AdapterType::Loopback => write!(f, "Local Loopback"),
            AdapterType::Tunnel => write!(f, "Tunnel"),
            AdapterType::PPP => write!(f, "Point-to-Point"),
            AdapterType::Bridge => write!(f, "Bridge"),
            AdapterType::VPN => write!(f, "VPN"),
            AdapterType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl AdapterType {
    pub fn emoji(&self) -> &str {
        match self {
            AdapterType::Ethernet => "🔌",
            AdapterType::WiFi => "📶",
            AdapterType::Loopback => "🔁",
            AdapterType::Tunnel => "🚇",
            AdapterType::PPP => "🔗",
            AdapterType::Bridge => "🌉",
            AdapterType::VPN => "🔒",
            AdapterType::Other(_) => "🔧",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdapterStatus {
    Up,
    Down,
    Testing,
    Unknown,
    Dormant,
    NotPresent,
    LowerLayerDown,
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterStatus::Up => write!(f, "UP"),
            AdapterStatus::Down => write!(f, "DOWN"),
            AdapterStatus::Testing => write!(f, "TESTING"),
            AdapterStatus::Unknown => write!(f, "UNKNOWN"),
            AdapterStatus::Dormant => write!(f, "DORMANT"),
            AdapterStatus::NotPresent => write!(f, "NOT-PRESENT"),
            AdapterStatus::LowerLayerDown => write!(f, "LOWER-LAYER-DOWN"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub rx_unicast: u64,
    pub rx_multicast: u64,
    pub rx_broadcast: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub tx_unicast: u64,
    pub tx_multicast: u64,
    pub tx_broadcast: u64,
    pub collisions: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterFlags {
    pub up: bool,
    pub broadcast: bool,
    pub debug: bool,
    pub loopback: bool,
    pub point_to_point: bool,
    pub running: bool,
    pub no_arp: bool,
    pub promisc: bool,
    pub multicast: bool,
    pub dynamic: bool,
    pub dhcp: bool,
}

impl AdapterFlags {
    pub fn to_flag_string(&self) -> String {
        let mut flags = Vec::new();
        if self.up {
            flags.push("UP");
        }
        if self.broadcast {
            flags.push("BROADCAST");
        }
        if self.loopback {
            flags.push("LOOPBACK");
        }
        if self.point_to_point {
            flags.push("POINTOPOINT");
        }
        if self.running {
            flags.push("RUNNING");
        }
        if self.no_arp {
            flags.push("NOARP");
        }
        if self.promisc {
            flags.push("PROMISC");
        }
        if self.multicast {
            flags.push("MULTICAST");
        }
        if self.dynamic {
            flags.push("DYNAMIC");
        }
        if self.dhcp {
            flags.push("DHCP");
        }
        flags.join(",")
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit_idx = 0;
    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.2} {}", value, UNITS[unit_idx])
    }
}

pub fn format_speed(bps: u64) -> String {
    if bps == 0 {
        return "unknown".to_string();
    }
    if bps >= 1_000_000_000 {
        format!("{:.0} Gbps", bps as f64 / 1_000_000_000.0)
    } else if bps >= 1_000_000 {
        format!("{:.0} Mbps", bps as f64 / 1_000_000.0)
    } else if bps >= 1_000 {
        format!("{:.0} Kbps", bps as f64 / 1_000.0)
    } else {
        format!("{} bps", bps)
    }
}
