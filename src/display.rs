// display.rs — Colorful terminal output for win-ifconfig
// Author: Hadi Cahyadi <cumulus13@gmail.com>

use crate::types::*;
use crate::Cli;
use colored::*;

const SEPARATOR_LEN: usize = 70;

/// Main text output function
pub fn output_text(adapters: &[AdapterInfo], cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Filter adapters
    let filtered: Vec<&AdapterInfo> = adapters
        .iter()
        .filter(|a| {
            // If a specific interface is named, filter to it
            if let Some(ref iface) = cli.interface {
                let iface_lower = iface.to_lowercase();
                return a.name.to_lowercase().contains(&iface_lower)
                    || a.friendly_name.to_lowercase().contains(&iface_lower)
                    || a.index.to_string() == *iface;
            }
            // --all flag shows everything, otherwise skip down adapters
            if !cli.all && !matches!(a.status, AdapterStatus::Up) {
                return false;
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        if let Some(ref iface) = cli.interface {
            eprintln!(
                "{}",
                format!("❌ Interface '{}' not found.", iface).red().bold()
            );
        } else {
            eprintln!(
                "{}",
                "❌ No active interfaces found. Use -a to show all.".red()
            );
        }
        return Ok(());
    }

    // Print header banner
    print_banner();

    // Print hostname line
    if let Ok(host) = hostname::get() {
        println!(
            "  {} {}   {} {}",
            "🖥️  Hostname:".bright_cyan().bold(),
            host.to_string_lossy().bright_white().bold(),
            "🕐".dimmed(),
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S %Z")
                .to_string()
                .bright_black()
        );
    }
    println!();

    for adapter in &filtered {
        if cli.brief {
            print_adapter_brief(adapter);
        } else {
            print_adapter_full(adapter, cli);
        }
    }

    if cli.summary {
        print_summary(adapters, &filtered);
    }

    Ok(())
}

fn print_banner() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════════╗"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}  {}  {}",
        "║".bright_cyan().bold(),
        format!(
            "🌐  win-ifconfig v{}  ─  Windows Network Interface Information",
            version
        )
        .bright_white()
        .bold(),
        "║".bright_cyan().bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════════╝"
            .bright_cyan()
            .bold()
    );
}

fn print_adapter_brief(adapter: &AdapterInfo) {
    let status_color = match adapter.status {
        AdapterStatus::Up => "UP".bright_green().bold(),
        AdapterStatus::Down => "DOWN".bright_red().bold(),
        _ => adapter.status.to_string().bright_yellow().bold(),
    };

    let type_emoji = adapter.adapter_type.emoji();
    let name = adapter.name.bright_white().bold();
    let mac = adapter
        .mac_address
        .as_deref()
        .unwrap_or("N/A")
        .bright_yellow();

    let ip = adapter
        .ipv4_addresses
        .first()
        .map(|a| a.address.as_str())
        .unwrap_or("No IP")
        .bright_green();

    println!(
        "{}  {}  {}  {}  {}",
        type_emoji, name, status_color, mac, ip
    );
}

fn print_adapter_full(adapter: &AdapterInfo, cli: &Cli) {
    // ─── Interface header ───────────────────────────────────────────────────
    let type_emoji = adapter.adapter_type.emoji();
    let status_str = match adapter.status {
        AdapterStatus::Up => "UP".bright_green().bold(),
        AdapterStatus::Down => "DOWN".bright_red().bold(),
        AdapterStatus::Dormant => "DORMANT".bright_yellow().bold(),
        _ => adapter.status.to_string().bright_yellow().bold(),
    };

    let flags_str = adapter.flags.to_flag_string();
    let mtu_str = format!("mtu {}", adapter.mtu);
    let metric_str = format!("metric {}", adapter.metric);

    println!(
        "{} {}: <{}> {} {} {}",
        type_emoji,
        adapter.name.bright_white().bold(),
        flags_str.bright_cyan(),
        status_str,
        mtu_str.bright_black(),
        metric_str.yellow()
    );

    // Description / type line
    println!(
        "        {}  {}",
        format!("link/{}", link_type_name(&adapter.adapter_type)).bright_black(),
        adapter.description.dimmed()
    );

    // MAC address
    if let Some(ref mac) = adapter.mac_address {
        println!(
            "        {}    {} {}",
            "ether".bright_black(),
            mac.bright_yellow().bold(),
            "txqueuelen 1000".bright_black()
        );
    }

    // Speed / duplex line
    if let Some(speed) = adapter.speed {
        println!(
            "        {}  {}",
            "speed".bright_black(),
            format_speed(speed).bright_magenta().bold()
        );
    }

    // ─── IPv4 addresses ────────────────────────────────────────────────────
    for ip in &adapter.ipv4_addresses {
        let scope_label = format!("scope {}", ip.scope);
        let nm = ip.netmask.as_deref().unwrap_or("");
        let bcast = ip.broadcast.as_deref().unwrap_or("");

        println!(
            "        {}   {}  netmask {}  broadcast {}  {}",
            "inet".bright_blue().bold(),
            ip.address.bright_green().bold(),
            nm.bright_black(),
            bcast.bright_black(),
            scope_label.dimmed()
        );
    }

    // ─── IPv6 addresses ────────────────────────────────────────────────────
    for ip in &adapter.ipv6_addresses {
        let scope_label = format!("scope {}", ip.scope);
        println!(
            "        {}  {}/{}  {}",
            "inet6".bright_blue().bold(),
            ip.address.bright_cyan().bold(),
            ip.prefix_len.to_string().bright_black(),
            scope_label.dimmed()
        );
    }

    // ─── Gateway ───────────────────────────────────────────────────────────
    if let Some(ref gw) = adapter.gateway {
        println!(
            "        {}     {}",
            "gateway".bright_black(),
            gw.bright_yellow()
        );
    }

    // ─── Stats (always shown unless brief) ─────────────────────────────────
    if let Some(ref stats) = adapter.stats {
        println!();
        println!(
            "        {} {} packets:{} errors:{} dropped:{} overruns:0 frame:0",
            "📥".dimmed(),
            "RX".bright_green().bold(),
            stats.rx_packets.to_string().bright_white(),
            if stats.rx_errors > 0 {
                stats.rx_errors.to_string().bright_red()
            } else {
                stats.rx_errors.to_string().bright_black()
            },
            if stats.rx_dropped > 0 {
                stats.rx_dropped.to_string().yellow()
            } else {
                stats.rx_dropped.to_string().bright_black()
            }
        );
        println!(
            "        {} {} bytes:{}  ({})",
            "   ".dimmed(),
            "RX".bright_green().bold(),
            stats.rx_bytes.to_string().bright_white(),
            format_bytes(stats.rx_bytes).bright_cyan()
        );

        if cli.stats || cli.verbose {
            println!(
                "               unicast:{} multicast:{} broadcast:{}",
                stats.rx_unicast.to_string().bright_black(),
                stats.rx_multicast.to_string().bright_black(),
                stats.rx_broadcast.to_string().bright_black()
            );
        }

        println!(
            "        {} {} packets:{} errors:{} dropped:{} overruns:0 carrier:0",
            "📤".dimmed(),
            "TX".bright_magenta().bold(),
            stats.tx_packets.to_string().bright_white(),
            if stats.tx_errors > 0 {
                stats.tx_errors.to_string().bright_red()
            } else {
                stats.tx_errors.to_string().bright_black()
            },
            if stats.tx_dropped > 0 {
                stats.tx_dropped.to_string().yellow()
            } else {
                stats.tx_dropped.to_string().bright_black()
            }
        );
        println!(
            "        {} {} bytes:{}  ({})",
            "   ".dimmed(),
            "TX".bright_magenta().bold(),
            stats.tx_bytes.to_string().bright_white(),
            format_bytes(stats.tx_bytes).bright_cyan()
        );

        if cli.stats || cli.verbose {
            println!(
                "               unicast:{} multicast:{} broadcast:{}",
                stats.tx_unicast.to_string().bright_black(),
                stats.tx_multicast.to_string().bright_black(),
                stats.tx_broadcast.to_string().bright_black()
            );
        }

        println!(
            "               collisions:{}",
            if stats.collisions > 0 {
                stats.collisions.to_string().bright_red()
            } else {
                stats.collisions.to_string().bright_black()
            }
        );
    }

    // ─── Extended Windows info ──────────────────────────────────────────────
    if cli.metrics || cli.verbose {
        println!();
        println!(
            "        {} {} {}",
            "📊".dimmed(),
            "METRIC:".bright_yellow().bold(),
            adapter.metric.to_string().bright_white()
        );
        if let Some(speed) = adapter.speed {
            println!(
                "        {} {} {}",
                "⚡".dimmed(),
                "SPEED:".bright_yellow().bold(),
                format_speed(speed).bright_magenta()
            );
        }
        println!(
            "        {} {} {}",
            "📦".dimmed(),
            "MTU:".bright_yellow().bold(),
            adapter.mtu.to_string().bright_white()
        );
        println!(
            "        {} {} {}",
            "🔢".dimmed(),
            "INDEX:".bright_yellow().bold(),
            adapter.index.to_string().bright_white()
        );
    }

    // ─── DHCP info ────────────────────────────────────────────────────────
    if cli.dhcp || cli.verbose {
        if adapter.dhcp_enabled {
            println!();
            println!(
                "        {} {} {}",
                "🏷️ ".dimmed(),
                "DHCP:".bright_cyan().bold(),
                "enabled".bright_green()
            );
            if let Some(ref srv) = adapter.dhcp_server {
                println!(
                    "        {} {} {}",
                    "   ".dimmed(),
                    "Server:".bright_black(),
                    srv.bright_white()
                );
            }
            if let Some(ref obtained) = adapter.dhcp_lease_obtained {
                println!(
                    "        {} {} {}",
                    "   ".dimmed(),
                    "Lease obtained:".bright_black(),
                    obtained.bright_white()
                );
            }
            if let Some(ref expires) = adapter.dhcp_lease_expires {
                println!(
                    "        {} {} {}",
                    "   ".dimmed(),
                    "Lease expires: ".bright_black(),
                    expires.bright_white()
                );
            }
        } else {
            println!();
            println!(
                "        {} {} {}",
                "🏷️ ".dimmed(),
                "DHCP:".bright_cyan().bold(),
                "disabled (static)".bright_yellow()
            );
        }
    }

    // ─── DNS info ─────────────────────────────────────────────────────────
    if (cli.dns || cli.verbose) && !adapter.dns_servers.is_empty() {
        println!();
        println!(
            "        {} {}",
            "🔍".dimmed(),
            "DNS servers:".bright_cyan().bold()
        );
        for dns in &adapter.dns_servers {
            println!("             {}", dns.bright_white());
        }
    }

    // ─── WINS servers ─────────────────────────────────────────────────────
    if cli.verbose && !adapter.wins_servers.is_empty() {
        println!(
            "        {} {}",
            "🔍".dimmed(),
            "WINS servers:".bright_cyan().bold()
        );
        for wins in &adapter.wins_servers {
            println!("             {}", wins.bright_white());
        }
    }

    // Separator line between adapters
    println!();
    println!("{}", "─".repeat(SEPARATOR_LEN).bright_black());
    println!();
}

fn print_summary(all_adapters: &[AdapterInfo], shown: &[&AdapterInfo]) {
    println!();
    println!("{}", "═".repeat(SEPARATOR_LEN).bright_cyan());
    println!(
        "  {} {}",
        "📋 SUMMARY".bright_white().bold(),
        "─ Network Interface Overview".bright_black()
    );
    println!("{}", "═".repeat(SEPARATOR_LEN).bright_cyan());

    let up_count = all_adapters
        .iter()
        .filter(|a| matches!(a.status, AdapterStatus::Up))
        .count();
    let down_count = all_adapters.len() - up_count;

    println!(
        "  Total interfaces: {}  │  Up: {}  │  Down: {}  │  Displayed: {}",
        all_adapters.len().to_string().bright_white().bold(),
        up_count.to_string().bright_green().bold(),
        down_count.to_string().bright_red().bold(),
        shown.len().to_string().bright_cyan().bold()
    );

    let total_rx: u64 = shown
        .iter()
        .filter_map(|a| a.stats.as_ref())
        .map(|s| s.rx_bytes)
        .sum();
    let total_tx: u64 = shown
        .iter()
        .filter_map(|a| a.stats.as_ref())
        .map(|s| s.tx_bytes)
        .sum();

    println!(
        "  Total RX: {}  │  Total TX: {}",
        format_bytes(total_rx).bright_green().bold(),
        format_bytes(total_tx).bright_magenta().bold()
    );
    println!();
}

/// JSON output
pub fn output_json(adapters: &[AdapterInfo], cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let filtered: Vec<&AdapterInfo> = adapters
        .iter()
        .filter(|a| {
            if let Some(ref iface) = cli.interface {
                let iface_lower = iface.to_lowercase();
                return a.name.to_lowercase().contains(&iface_lower)
                    || a.index.to_string() == *iface;
            }
            if !cli.all && !matches!(a.status, AdapterStatus::Up) {
                return false;
            }
            true
        })
        .collect();

    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{}", json);
    Ok(())
}

fn link_type_name(t: &AdapterType) -> &str {
    match t {
        AdapterType::Ethernet => "ether",
        AdapterType::WiFi => "ieee802.11",
        AdapterType::Loopback => "loopback",
        AdapterType::Tunnel => "tunnel",
        AdapterType::Ppp => "ppp",
        AdapterType::Bridge => "bridge",
        AdapterType::Vpn => "vpn",
        AdapterType::Other(_) => "unknown",
    }
}
