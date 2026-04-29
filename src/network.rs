// network.rs — Windows network adapter data collection
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Uses the Windows IP Helper API (iphlpapi) via the `windows` crate

use crate::types::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[cfg(windows)]
use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
#[cfg(windows)]
use windows::Win32::NetworkManagement::IpHelper::{
    GetAdaptersAddresses, GetIfEntry2, GAA_FLAG_INCLUDE_ALL_INTERFACES, GAA_FLAG_INCLUDE_GATEWAYS,
    GAA_FLAG_INCLUDE_PREFIX, GAA_FLAG_INCLUDE_WINS_INFO, IP_ADAPTER_ADDRESSES_LH, MIB_IF_ROW2,
};
#[cfg(windows)]
use windows::Win32::Networking::WinSock::{AF_UNSPEC, SOCKET_ADDRESS};

/// Main entry point: collect all adapter information
pub fn get_adapters() -> Result<Vec<AdapterInfo>, Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        get_adapters_windows()
    }
    #[cfg(not(windows))]
    {
        // Stub for non-Windows compilation (CI on Linux)
        Ok(get_stub_adapters())
    }
}

#[cfg(not(windows))]
fn get_stub_adapters() -> Vec<AdapterInfo> {
    // Return stub data when compiled on non-Windows for testing
    vec![
        AdapterInfo {
            index: 1,
            name: "lo".to_string(),
            friendly_name: "Loopback".to_string(),
            description: "Software Loopback Interface 1".to_string(),
            adapter_type: AdapterType::Loopback,
            status: AdapterStatus::Up,
            mac_address: None,
            ipv4_addresses: vec![IpAddress {
                address: "127.0.0.1".to_string(),
                prefix_len: 8,
                netmask: Some("255.0.0.0".to_string()),
                broadcast: None,
                scope: IpScope::Loopback,
            }],
            ipv6_addresses: vec![IpAddress {
                address: "::1".to_string(),
                prefix_len: 128,
                netmask: None,
                broadcast: None,
                scope: IpScope::Loopback,
            }],
            gateway: None,
            dns_servers: vec![],
            dhcp_enabled: false,
            dhcp_server: None,
            dhcp_lease_obtained: None,
            dhcp_lease_expires: None,
            mtu: 65536,
            speed: None,
            metric: 75,
            stats: Some(AdapterStats {
                rx_bytes: 1_234_567,
                rx_packets: 12_345,
                rx_errors: 0,
                rx_dropped: 0,
                rx_unicast: 12_000,
                rx_multicast: 200,
                rx_broadcast: 145,
                tx_bytes: 1_234_567,
                tx_packets: 12_345,
                tx_errors: 0,
                tx_dropped: 0,
                tx_unicast: 12_000,
                tx_multicast: 200,
                tx_broadcast: 145,
                collisions: 0,
            }),
            flags: AdapterFlags {
                up: true,
                broadcast: false,
                debug: false,
                loopback: true,
                point_to_point: false,
                running: true,
                no_arp: true,
                promisc: false,
                multicast: true,
                dynamic: false,
                dhcp: false,
            },
            wins_servers: vec![],
        },
        AdapterInfo {
            index: 2,
            name: "Ethernet".to_string(),
            friendly_name: "Ethernet".to_string(),
            description: "Intel(R) Ethernet Connection I219-V".to_string(),
            adapter_type: AdapterType::Ethernet,
            status: AdapterStatus::Up,
            mac_address: Some("A4:BB:6D:23:91:F0".to_string()),
            ipv4_addresses: vec![IpAddress {
                address: "192.168.1.100".to_string(),
                prefix_len: 24,
                netmask: Some("255.255.255.0".to_string()),
                broadcast: Some("192.168.1.255".to_string()),
                scope: IpScope::Global,
            }],
            ipv6_addresses: vec![IpAddress {
                address: "fe80::1a2b:3c4d:5e6f:7890".to_string(),
                prefix_len: 64,
                netmask: None,
                broadcast: None,
                scope: IpScope::LinkLocal,
            }],
            gateway: Some("192.168.1.1".to_string()),
            dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            dhcp_enabled: true,
            dhcp_server: Some("192.168.1.1".to_string()),
            dhcp_lease_obtained: Some("2024-01-15 08:00:00".to_string()),
            dhcp_lease_expires: Some("2024-01-16 08:00:00".to_string()),
            mtu: 1500,
            speed: Some(1_000_000_000),
            metric: 25,
            stats: Some(AdapterStats {
                rx_bytes: 9_876_543_210,
                rx_packets: 7_654_321,
                rx_errors: 3,
                rx_dropped: 12,
                rx_unicast: 7_000_000,
                rx_multicast: 50_000,
                rx_broadcast: 604_321,
                tx_bytes: 1_234_567_890,
                tx_packets: 1_234_567,
                tx_errors: 0,
                tx_dropped: 1,
                tx_unicast: 1_200_000,
                tx_multicast: 10_000,
                tx_broadcast: 24_567,
                collisions: 0,
            }),
            flags: AdapterFlags {
                up: true,
                broadcast: true,
                debug: false,
                loopback: false,
                point_to_point: false,
                running: true,
                no_arp: false,
                promisc: false,
                multicast: true,
                dynamic: false,
                dhcp: true,
            },
            wins_servers: vec![],
        },
        AdapterInfo {
            index: 3,
            name: "Wi-Fi".to_string(),
            friendly_name: "Wi-Fi".to_string(),
            description: "Intel(R) Wi-Fi 6 AX200 160MHz".to_string(),
            adapter_type: AdapterType::WiFi,
            status: AdapterStatus::Down,
            mac_address: Some("D0:AB:D5:CC:72:11".to_string()),
            ipv4_addresses: vec![],
            ipv6_addresses: vec![],
            gateway: None,
            dns_servers: vec![],
            dhcp_enabled: true,
            dhcp_server: None,
            dhcp_lease_obtained: None,
            dhcp_lease_expires: None,
            mtu: 1500,
            speed: Some(1_201_000_000),
            metric: 35,
            stats: Some(AdapterStats {
                rx_bytes: 0,
                rx_packets: 0,
                rx_errors: 0,
                rx_dropped: 0,
                rx_unicast: 0,
                rx_multicast: 0,
                rx_broadcast: 0,
                tx_bytes: 0,
                tx_packets: 0,
                tx_errors: 0,
                tx_dropped: 0,
                tx_unicast: 0,
                tx_multicast: 0,
                tx_broadcast: 0,
                collisions: 0,
            }),
            flags: AdapterFlags {
                up: false,
                broadcast: true,
                debug: false,
                loopback: false,
                point_to_point: false,
                running: false,
                no_arp: false,
                promisc: false,
                multicast: true,
                dynamic: false,
                dhcp: true,
            },
            wins_servers: vec![],
        },
    ]
}

#[cfg(windows)]
fn get_adapters_windows() -> Result<Vec<AdapterInfo>, Box<dyn std::error::Error>> {
    let flags = GAA_FLAG_INCLUDE_PREFIX
        | GAA_FLAG_INCLUDE_WINS_INFO
        | GAA_FLAG_INCLUDE_GATEWAYS
        | GAA_FLAG_INCLUDE_ALL_INTERFACES;

    let mut buf_len: u32 = 16384;
    let mut buf: Vec<u8>;

    loop {
        buf = vec![0u8; buf_len as usize];
        let result = unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC.0 as u32,
                flags,
                None,
                Some(buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                &mut buf_len,
            )
        };

        if result == 0 {
            break;
        } else if result == ERROR_BUFFER_OVERFLOW.0 {
            // buf_len was updated by the call, retry with larger buffer
            continue;
        } else {
            return Err(format!("GetAdaptersAddresses failed with error: {}", result).into());
        }
    }

    let mut adapters = Vec::new();
    let mut current = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;

    while !current.is_null() {
        let adapter = unsafe { &*current };
        if let Some(info) = parse_adapter(adapter) {
            adapters.push(info);
        }
        current = adapter.Next;
    }

    Ok(adapters)
}

#[cfg(windows)]
fn parse_adapter(adapter: &IP_ADAPTER_ADDRESSES_LH) -> Option<AdapterInfo> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows::Win32::NetworkManagement::IpHelper::{
        IF_TYPE_ETHERNET_CSMACD, IF_TYPE_IEEE80211, IF_TYPE_PPP, IF_TYPE_SOFTWARE_LOOPBACK,
        IF_TYPE_TUNNEL,
    };
    use windows::Win32::NetworkManagement::Ndis::IfOperStatusUp;

    // Adapter name (GUID style)
    let name = unsafe {
        if adapter.AdapterName.0.is_null() {
            return None;
        }
        let s = std::ffi::CStr::from_ptr(adapter.AdapterName.0 as *const i8);
        s.to_string_lossy().to_string()
    };

    // Friendly name (human-readable like "Ethernet", "Wi-Fi")
    let friendly_name = unsafe {
        if adapter.FriendlyName.0.is_null() {
            name.clone()
        } else {
            let len = (0..)
                .take_while(|&i| *adapter.FriendlyName.0.add(i) != 0)
                .count();
            let slice = std::slice::from_raw_parts(adapter.FriendlyName.0, len);
            OsString::from_wide(slice).to_string_lossy().to_string()
        }
    };

    // Description
    let description = unsafe {
        if adapter.Description.0.is_null() {
            String::new()
        } else {
            let len = (0..)
                .take_while(|&i| *adapter.Description.0.add(i) != 0)
                .count();
            let slice = std::slice::from_raw_parts(adapter.Description.0, len);
            OsString::from_wide(slice).to_string_lossy().to_string()
        }
    };

    // Adapter type
    let adapter_type = match adapter.IfType {
        t if t == IF_TYPE_ETHERNET_CSMACD => AdapterType::Ethernet,
        t if t == IF_TYPE_IEEE80211 => AdapterType::WiFi,
        t if t == IF_TYPE_SOFTWARE_LOOPBACK => AdapterType::Loopback,
        t if t == IF_TYPE_TUNNEL => AdapterType::Tunnel,
        t if t == IF_TYPE_PPP => AdapterType::Ppp,
        _ => {
            if description.to_lowercase().contains("vpn")
                || description.to_lowercase().contains("virtual")
            {
                AdapterType::Vpn
            } else {
                AdapterType::Other(format!("Type({})", adapter.IfType))
            }
        }
    };

    // Status
    let status = match adapter.OperStatus {
        s if s == IfOperStatusUp => AdapterStatus::Up,
        _ => AdapterStatus::Down,
    };

    // MAC address
    let mac_address = if adapter.PhysicalAddressLength > 0 {
        let bytes = &adapter.PhysicalAddress[..adapter.PhysicalAddressLength as usize];
        Some(
            bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(":"),
        )
    } else {
        None
    };

    // IPv4 and IPv6 addresses
    let mut ipv4_addresses = Vec::new();
    let mut ipv6_addresses = Vec::new();

    let mut unicast = adapter.FirstUnicastAddress;
    while !unicast.is_null() {
        let ua = unsafe { &*unicast };
        if let Some(ip) = parse_socket_address(&ua.Address) {
            let prefix_len = ua.OnLinkPrefixLength;
            match ip {
                IpAddr::V4(v4) => {
                    let netmask = prefix_to_netmask_v4(prefix_len);
                    let broadcast = calc_broadcast_v4(v4, prefix_len);
                    let scope = if v4.is_loopback() {
                        IpScope::Loopback
                    } else if v4.is_link_local() {
                        IpScope::LinkLocal
                    } else {
                        IpScope::Global
                    };
                    ipv4_addresses.push(IpAddress {
                        address: v4.to_string(),
                        prefix_len,
                        netmask: Some(netmask),
                        broadcast: Some(broadcast),
                        scope,
                    });
                }
                IpAddr::V6(v6) => {
                    let scope = classify_ipv6_scope(v6);
                    ipv6_addresses.push(IpAddress {
                        address: v6.to_string(),
                        prefix_len,
                        netmask: None,
                        broadcast: None,
                        scope,
                    });
                }
            }
        }
        unicast = ua.Next;
    }

    // Gateway
    let mut gateway = None;
    let mut gw_ptr = adapter.FirstGatewayAddress;
    while !gw_ptr.is_null() {
        let gw = unsafe { &*gw_ptr };
        if let Some(ip) = parse_socket_address(&gw.Address) {
            if ip.is_ipv4() {
                gateway = Some(ip.to_string());
                break;
            }
        }
        gw_ptr = gw.Next;
    }

    // DNS servers — try FirstDnsServerAddress first (works for static DNS)
    let mut dns_servers = Vec::new();
    let mut dns_ptr = adapter.FirstDnsServerAddress;
    while !dns_ptr.is_null() {
        let dns = unsafe { &*dns_ptr };
        if let Some(ip) = parse_socket_address(&dns.Address) {
            dns_servers.push(ip.to_string());
        }
        dns_ptr = dns.Next;
    }

    // DHCP — the Flags field is inside an anonymous union: adapter.Anonymous2.Flags
    let dhcp_enabled = (unsafe { adapter.Anonymous2.Flags } & 0x04) != 0;

    // DHCP server — Dhcpv4Server is a SOCKET_ADDRESS value (not a pointer)
    let dhcp_server = parse_socket_address(&adapter.Dhcpv4Server).map(|ip| ip.to_string());

    // For DHCP-assigned DNS, FirstDnsServerAddress is often empty on Windows.
    // Fall back to the registry (keyed by adapter GUID), which always has current DNS.
    if dns_servers.is_empty() && !name.is_empty() {
        if let Ok(reg_dns) = read_dns_from_registry(&name) {
            dns_servers = reg_dns;
        }
    }

    // DHCP lease times from registry (keyed by adapter GUID)
    let (dhcp_lease_obtained, dhcp_lease_expires) = read_lease_times_from_registry(&name);

    // WINS servers
    let mut wins_servers = Vec::new();
    let mut wins = adapter.FirstWinsServerAddress;
    while !wins.is_null() {
        let w = unsafe { &*wins };
        if let Some(ip) = parse_socket_address(&w.Address) {
            wins_servers.push(ip.to_string());
        }
        wins = w.Next;
    }

    // IfIndex lives inside Anonymous1.Anonymous.IfIndex
    let if_index = unsafe { adapter.Anonymous1.Anonymous.IfIndex };
    let stats = get_interface_stats(if_index);

    // Speed
    let speed = if adapter.TransmitLinkSpeed > 0 && adapter.TransmitLinkSpeed != u64::MAX {
        Some(adapter.TransmitLinkSpeed)
    } else {
        None
    };

    let is_loopback = matches!(adapter_type, AdapterType::Loopback);
    let is_up = matches!(status, AdapterStatus::Up);
    let is_ppp = matches!(adapter_type, AdapterType::Ppp);
    let is_dhcp = dhcp_enabled;

    Some(AdapterInfo {
        index: if_index,
        name: friendly_name.clone(),
        friendly_name,
        description,
        adapter_type,
        status,
        mac_address,
        ipv4_addresses,
        ipv6_addresses,
        gateway,
        dns_servers,
        dhcp_enabled,
        dhcp_server,
        dhcp_lease_obtained,
        dhcp_lease_expires,
        mtu: adapter.Mtu,
        speed,
        metric: adapter.Ipv4Metric,
        stats,
        flags: AdapterFlags {
            up: is_up,
            broadcast: !is_loopback && !is_ppp,
            debug: false,
            loopback: is_loopback,
            point_to_point: is_ppp,
            running: is_up,
            no_arp: is_loopback,
            promisc: false,
            multicast: !is_loopback,
            dynamic: false,
            dhcp: is_dhcp,
        },
        wins_servers,
    })
}

#[cfg(windows)]
fn parse_socket_address(addr: &SOCKET_ADDRESS) -> Option<IpAddr> {
    use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6, SOCKADDR_IN, SOCKADDR_IN6};

    if addr.lpSockaddr.is_null() || addr.iSockaddrLength == 0 {
        return None;
    }

    let sa = unsafe { &*addr.lpSockaddr };
    match sa.sa_family {
        f if f == AF_INET => {
            let sin = unsafe { &*(addr.lpSockaddr as *const SOCKADDR_IN) };
            let ip_bytes = unsafe { sin.sin_addr.S_un.S_un_b };
            Some(IpAddr::V4(Ipv4Addr::new(
                ip_bytes.s_b1,
                ip_bytes.s_b2,
                ip_bytes.s_b3,
                ip_bytes.s_b4,
            )))
        }
        f if f == AF_INET6 => {
            let sin6 = unsafe { &*(addr.lpSockaddr as *const SOCKADDR_IN6) };
            let bytes = unsafe { sin6.sin6_addr.u.Byte };
            Some(IpAddr::V6(Ipv6Addr::from(bytes)))
        }
        _ => None,
    }
}

#[cfg(windows)]
fn get_interface_stats(if_index: u32) -> Option<AdapterStats> {
    use windows::Win32::Foundation::WIN32_ERROR;

    let mut row = MIB_IF_ROW2 {
        InterfaceIndex: if_index,
        ..Default::default()
    };

    let result = unsafe { GetIfEntry2(&mut row) };
    if result != WIN32_ERROR(0) {
        return None;
    }

    Some(AdapterStats {
        rx_bytes: row.InOctets,
        rx_packets: row.InUcastPkts + row.InNUcastPkts,
        rx_errors: row.InErrors,
        rx_dropped: row.InDiscards,
        rx_unicast: row.InUcastPkts,
        // MIB_IF_ROW2 exposes Octets (bytes) for multicast/broadcast, not packet counts
        rx_multicast: row.InMulticastOctets,
        rx_broadcast: row.InBroadcastOctets,
        tx_bytes: row.OutOctets,
        tx_packets: row.OutUcastPkts + row.OutNUcastPkts,
        tx_errors: row.OutErrors,
        tx_dropped: row.OutDiscards,
        tx_unicast: row.OutUcastPkts,
        tx_multicast: row.OutMulticastOctets,
        tx_broadcast: row.OutBroadcastOctets,
        collisions: 0, // Not available in MIB_IF_ROW2
    })
}

fn prefix_to_netmask_v4(prefix_len: u8) -> String {
    if prefix_len == 0 {
        return "0.0.0.0".to_string();
    }
    let mask: u32 = if prefix_len >= 32 {
        0xFFFF_FFFF
    } else {
        !((1u32 << (32 - prefix_len)) - 1)
    };
    Ipv4Addr::from(mask).to_string()
}

fn calc_broadcast_v4(addr: Ipv4Addr, prefix_len: u8) -> String {
    let ip: u32 = u32::from(addr);
    let mask: u32 = if prefix_len >= 32 {
        0xFFFF_FFFF
    } else {
        !((1u32 << (32 - prefix_len)) - 1)
    };
    let bcast = (ip & mask) | !mask;
    Ipv4Addr::from(bcast).to_string()
}

fn classify_ipv6_scope(addr: Ipv6Addr) -> IpScope {
    let segs = addr.segments();
    if addr.is_loopback() {
        IpScope::Loopback
    } else if (segs[0] & 0xFFC0) == 0xFE80 {
        IpScope::LinkLocal
    } else if (segs[0] & 0xFFC0) == 0xFEC0 {
        IpScope::SiteLocal
    } else {
        IpScope::Global
    }
}

#[allow(dead_code)]
fn filetime_to_string(ft: i64) -> Option<String> {
    if ft == 0 {
        return None;
    }
    // Windows FILETIME: 100-nanosecond intervals since January 1, 1601
    let unix_epoch_offset: i64 = 116_444_736_000_000_000;
    let unix_ts_100ns = ft - unix_epoch_offset;
    if unix_ts_100ns <= 0 {
        return None;
    }
    let unix_ts_secs = unix_ts_100ns / 10_000_000;
    use chrono::{TimeZone, Utc};
    let dt = Utc.timestamp_opt(unix_ts_secs, 0).single()?;
    let local: chrono::DateTime<chrono::Local> = dt.into();
    Some(local.format("%Y-%m-%d %H:%M:%S").to_string())
}

// ─── Registry helpers for DNS and DHCP lease times ───────────────────────────
// Windows stores per-adapter network config under:
//   HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{GUID}
// Key fields:
//   NameServer        — static DNS (comma or space separated)
//   DhcpNameServer    — DHCP-assigned DNS (space separated)
//   LeaseObtainedTime — DHCP lease obtained (Unix timestamp, REG_DWORD)
//   LeaseTerminatesTime — DHCP lease expiry (Unix timestamp, REG_DWORD)

#[cfg(windows)]
fn read_dns_from_registry(
    guid: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    let key_path = format!(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}",
        guid
    );
    let key_path_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();

    let mut hkey = windows::Win32::System::Registry::HKEY::default();
    let result = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(key_path_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        )
    };
    if result.is_err() {
        return Err("Registry key not found".into());
    }

    let servers = read_reg_dns_value(hkey, "DhcpNameServer")
        .or_else(|_| read_reg_dns_value(hkey, "NameServer"));

    unsafe { let _ = RegCloseKey(hkey).ok(); };

    servers
}

#[cfg(windows)]
fn read_reg_dns_value(
    hkey: windows::Win32::System::Registry::HKEY,
    value_name: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegQueryValueExW, REG_VALUE_TYPE};

    let name_wide: Vec<u16> = value_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut data_type = REG_VALUE_TYPE(0);
    let mut data_len: u32 = 0;

    // First call: get size
    let _ = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(name_wide.as_ptr()),
            None,
            Some(&mut data_type),
            None,
            Some(&mut data_len),
        )
    };

    if data_len == 0 {
        return Err("Empty value".into());
    }

    let mut buf = vec![0u16; (data_len as usize / 2) + 1];
    let result = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(name_wide.as_ptr()),
            None,
            Some(&mut data_type),
            Some(buf.as_mut_ptr() as *mut u8),
            Some(&mut data_len),
        )
    };

    if result.is_err() {
        return Err("Failed to read registry value".into());
    }

    // Trim null terminators
    while buf.last() == Some(&0) {
        buf.pop();
    }

    let s = String::from_utf16_lossy(&buf);
    if s.is_empty() {
        return Err("Empty DNS string".into());
    }

    // Windows separates DNS with spaces or commas
    let servers: Vec<String> = s
        .split(|c| c == ' ' || c == ',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if servers.is_empty() {
        return Err("No DNS servers found".into());
    }

    Ok(servers)
}

#[cfg(windows)]
fn read_lease_times_from_registry(guid: &str) -> (Option<String>, Option<String>) {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{
        RegCloseKey, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    let key_path = format!(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}",
        guid
    );
    let key_path_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();

    let mut hkey = windows::Win32::System::Registry::HKEY::default();
    let result = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(key_path_wide.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        )
    };
    if result.is_err() {
        return (None, None);
    }

    let obtained = read_reg_unix_timestamp(hkey, "LeaseObtainedTime");
    let expires = read_reg_unix_timestamp(hkey, "LeaseTerminatesTime");

    unsafe { let _ = RegCloseKey(hkey).ok(); };

    (obtained, expires)
}

#[cfg(windows)]
fn read_reg_unix_timestamp(
    hkey: windows::Win32::System::Registry::HKEY,
    value_name: &str,
) -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegQueryValueExW, REG_VALUE_TYPE};

    let name_wide: Vec<u16> = value_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut data_type = REG_VALUE_TYPE(0);
    let mut value: u32 = 0;
    let mut data_len: u32 = 4;

    let result = unsafe {
        RegQueryValueExW(
            hkey,
            PCWSTR(name_wide.as_ptr()),
            None,
            Some(&mut data_type),
            Some(&mut value as *mut u32 as *mut u8),
            Some(&mut data_len),
        )
    };

    if result.is_err() || value == 0 {
        return None;
    }

    // Value is a Unix timestamp (seconds since 1970-01-01)
    use chrono::{DateTime, Local, TimeZone, Utc};
    let dt = Utc.timestamp_opt(value as i64, 0).single()?;
    let local: DateTime<Local> = dt.into();
    Some(local.format("%Y-%m-%d %H:%M:%S").to_string())
}

// Non-Windows stubs
#[cfg(not(windows))]
fn read_dns_from_registry(
    _guid: &str,
) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    Err("Registry not available on non-Windows".into())
}

#[cfg(not(windows))]
fn read_lease_times_from_registry(_guid: &str) -> (Option<String>, Option<String>) {
    (None, None)
}
