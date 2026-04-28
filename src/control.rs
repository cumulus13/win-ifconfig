// control.rs — Network interface configuration for win-ifconfig
// Author: Hadi Cahyadi <cumulus13@gmail.com>
//
// All write operations are performed through:
//   • netsh interface ip / ipv4   — IP, gateway, DNS, MTU
//   • netsh interface set         — enable/disable, metric
//   • PowerShell Set-NetAdapter   — MAC address
//   • PowerShell Set-NetIPInterface — MTU, metric
//
// All commands require Administrator privileges.

use crate::SetOperation;
use colored::*;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// ─── Public entry points ──────────────────────────────────────────────────────

pub fn interface_up(iface: &str) -> Result<()> {
    println!(
        "{}  Bringing {} {}...",
        "⬆️ ".bright_green(),
        iface.bright_white().bold(),
        "UP".bright_green().bold()
    );
    netsh(&["interface", "set", "interface", iface, "admin=enabled"])?;
    println!("  {} Interface enabled.", "✅".bright_green());
    Ok(())
}

pub fn interface_down(iface: &str) -> Result<()> {
    println!(
        "{}  Bringing {} {}...",
        "⬇️ ".bright_red(),
        iface.bright_white().bold(),
        "DOWN".bright_red().bold()
    );
    netsh(&["interface", "set", "interface", iface, "admin=disabled"])?;
    println!("  {} Interface disabled.", "✅".bright_green());
    Ok(())
}

pub fn run_set(iface: &str, op: &SetOperation) -> Result<()> {
    match op {
        SetOperation::Ip(args) => set_ip(iface, &args.address, args.gateway.as_deref()),
        SetOperation::Mtu { value } => set_mtu(iface, *value),
        SetOperation::Metric { value } => set_metric(iface, *value),
        SetOperation::Dns(args) => set_dns(iface, &args.servers),
        SetOperation::Add { address } => add_ip(iface, address),
        SetOperation::Del { address } => del_ip(iface, address),
        SetOperation::Mac { address } => set_mac(iface, address),
        SetOperation::Flush => flush_ip(iface),
    }
}

// ─── IP address ──────────────────────────────────────────────────────────────

fn set_ip(iface: &str, address: &str, gateway: Option<&str>) -> Result<()> {
    if address.eq_ignore_ascii_case("dhcp") {
        println!(
            "{}  Setting {} to {}...",
            "🔄".cyan(),
            iface.bright_white().bold(),
            "DHCP".bright_cyan().bold()
        );
        // Enable DHCP for address
        netsh(&[
            "interface",
            "ip",
            "set",
            "address",
            iface,
            "dhcp",
        ])?;
        println!("  {} DHCP enabled for address.", "✅".bright_green());
        return Ok(());
    }

    // Parse CIDR  e.g. "192.168.1.50/24"
    let (ip, prefix) = parse_cidr(address)?;
    let mask = prefix_to_mask(prefix);

    println!(
        "{}  Setting {} → ip:{} mask:{} {}",
        "🔧".yellow(),
        iface.bright_white().bold(),
        ip.bright_green(),
        mask.bright_black(),
        gateway
            .map(|g| format!("gw:{}", g.bright_yellow()))
            .unwrap_or_default()
    );

    if let Some(gw) = gateway {
        netsh(&[
            "interface",
            "ip",
            "set",
            "address",
            iface,
            "static",
            &ip,
            &mask,
            gw,
        ])?;
    } else {
        netsh(&[
            "interface",
            "ip",
            "set",
            "address",
            iface,
            "static",
            &ip,
            &mask,
        ])?;
    }

    println!("  {} IP address set.", "✅".bright_green());
    Ok(())
}

fn add_ip(iface: &str, address: &str) -> Result<()> {
    let (ip, prefix) = parse_cidr(address)?;
    let mask = prefix_to_mask(prefix);

    println!(
        "{}  Adding {} to {}...",
        "➕".bright_green(),
        address.bright_green(),
        iface.bright_white().bold()
    );

    netsh(&[
        "interface",
        "ip",
        "add",
        "address",
        iface,
        &ip,
        &mask,
    ])?;

    println!("  {} Address added.", "✅".bright_green());
    Ok(())
}

fn del_ip(iface: &str, address: &str) -> Result<()> {
    // Strip prefix if user supplied one
    let ip = address.split('/').next().unwrap_or(address);

    println!(
        "{}  Removing {} from {}...",
        "➖".bright_red(),
        ip.bright_red(),
        iface.bright_white().bold()
    );

    netsh(&[
        "interface",
        "ip",
        "delete",
        "address",
        iface,
        ip,
    ])?;

    println!("  {} Address removed.", "✅".bright_green());
    Ok(())
}

fn flush_ip(iface: &str) -> Result<()> {
    println!(
        "{}  Flushing all IPs on {} and resetting to DHCP...",
        "🔄".cyan(),
        iface.bright_white().bold()
    );

    // Reset to DHCP — this removes all static addresses
    netsh(&[
        "interface",
        "ip",
        "set",
        "address",
        iface,
        "dhcp",
    ])?;

    netsh(&[
        "interface",
        "ip",
        "set",
        "dns",
        iface,
        "dhcp",
    ])?;

    println!(
        "  {} Interface flushed — DHCP re-enabled for IP and DNS.",
        "✅".bright_green()
    );
    Ok(())
}

// ─── MTU ─────────────────────────────────────────────────────────────────────

fn set_mtu(iface: &str, mtu: u32) -> Result<()> {
    println!(
        "{}  Setting MTU on {} → {}...",
        "📦".yellow(),
        iface.bright_white().bold(),
        mtu.to_string().bright_cyan().bold()
    );

    // netsh interface ipv4 set subinterface <iface> mtu=<N> store=persistent
    netsh(&[
        "interface",
        "ipv4",
        "set",
        "subinterface",
        iface,
        &format!("mtu={}", mtu),
        "store=persistent",
    ])?;

    println!("  {} MTU set to {}.", "✅".bright_green(), mtu);
    Ok(())
}

// ─── Metric ──────────────────────────────────────────────────────────────────

fn set_metric(iface: &str, metric: u32) -> Result<()> {
    println!(
        "{}  Setting metric on {} → {}...",
        "📊".yellow(),
        iface.bright_white().bold(),
        metric.to_string().bright_cyan().bold()
    );

    // Disable automatic metric first, then set value
    netsh(&[
        "interface",
        "ip",
        "set",
        "interface",
        iface,
        "metric=automatic",
    ])
    .ok(); // ignore if not supported

    netsh(&[
        "interface",
        "ip",
        "set",
        "interface",
        iface,
        &format!("metric={}", metric),
    ])?;

    println!("  {} Metric set to {}.", "✅".bright_green(), metric);
    Ok(())
}

// ─── DNS ─────────────────────────────────────────────────────────────────────

fn set_dns(iface: &str, servers: &[String]) -> Result<()> {
    if servers.len() == 1 && servers[0].eq_ignore_ascii_case("dhcp") {
        println!(
            "{}  Setting DNS on {} → {}...",
            "🔍".cyan(),
            iface.bright_white().bold(),
            "DHCP (automatic)".bright_cyan()
        );
        netsh(&["interface", "ip", "set", "dns", iface, "dhcp"])?;
        println!("  {} DNS set to automatic.", "✅".bright_green());
        return Ok(());
    }

    println!(
        "{}  Setting DNS on {} → {}...",
        "🔍".yellow(),
        iface.bright_white().bold(),
        servers
            .iter()
            .map(|s| s.bright_white().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Set primary DNS
    netsh(&[
        "interface",
        "ip",
        "set",
        "dns",
        iface,
        "static",
        &servers[0],
        "primary",
    ])?;

    // Add additional DNS servers
    for (i, srv) in servers.iter().enumerate().skip(1) {
        let idx = (i + 1).to_string();
        netsh(&[
            "interface",
            "ip",
            "add",
            "dns",
            iface,
            srv,
            &format!("index={}", idx),
        ])?;
    }

    println!(
        "  {} DNS configured ({} server{}).",
        "✅".bright_green(),
        servers.len(),
        if servers.len() == 1 { "" } else { "s" }
    );
    Ok(())
}

// ─── MAC address ─────────────────────────────────────────────────────────────

fn set_mac(iface: &str, mac: &str) -> Result<()> {
    // Validate MAC format
    let mac_clean = mac.replace(':', "").replace('-', "");
    if mac_clean.len() != 12 || !mac_clean.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "Invalid MAC address '{}'. Use format AA:BB:CC:DD:EE:FF",
            mac
        )
        .into());
    }

    println!(
        "{}  Setting MAC on {} → {}...",
        "🔌".yellow(),
        iface.bright_white().bold(),
        mac.bright_yellow().bold()
    );

    // PowerShell: Set-NetAdapter -Name <iface> -MacAddress <mac>
    // The adapter must support software MAC override (most do)
    let ps_cmd = format!(
        "Set-NetAdapter -Name '{}' -MacAddress '{}' -Confirm:$false",
        iface, mac
    );

    powershell(&ps_cmd)?;

    println!(
        "  {} MAC address changed to {}.",
        "✅".bright_green(),
        mac.bright_yellow()
    );
    println!(
        "  {} You may need to reconnect the interface for the change to take effect.",
        "ℹ️ ".bright_cyan()
    );
    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Run a netsh command and capture output. Returns error on non-zero exit.
fn netsh(args: &[&str]) -> Result<String> {
    let output = Command::new("netsh").args(args).output().map_err(|e| {
        format!(
            "Failed to run netsh (is it in PATH?): {}",
            e
        )
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        let msg = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else if !stdout.trim().is_empty() {
            stdout.trim().to_string()
        } else {
            format!("netsh exited with code {:?}", output.status.code())
        };
        return Err(format!("netsh error: {}", msg).into());
    }

    // netsh sometimes exits 0 but prints an error in stdout
    let out_lower = stdout.to_lowercase();
    if out_lower.contains("the parameter is incorrect")
        || out_lower.contains("access is denied")
        || out_lower.contains("failed")
        || out_lower.contains("error")
    {
        // Filter out benign lines and only fail on real errors
        let error_lines: Vec<&str> = stdout
            .lines()
            .filter(|l| {
                let ll = l.to_lowercase();
                ll.contains("error")
                    || ll.contains("access is denied")
                    || ll.contains("the parameter is incorrect")
                    || ll.contains("failed")
            })
            .collect();

        if !error_lines.is_empty() {
            return Err(format!(
                "netsh reported: {}{}",
                error_lines.join("; "),
                if stdout.to_lowercase().contains("access is denied")
                    || stdout.to_lowercase().contains("administrator")
                {
                    "\n  💡 Tip: Run as Administrator"
                } else {
                    ""
                }
            )
            .into());
        }
    }

    Ok(stdout)
}

/// Run a PowerShell command. Returns error on non-zero exit.
fn powershell(cmd: &str) -> Result<String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", cmd])
        .output()
        .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        let msg = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else {
            stdout.trim().to_string()
        };
        return Err(format!(
            "PowerShell error: {}{}",
            msg,
            if msg.to_lowercase().contains("access")
                || msg.to_lowercase().contains("privilege")
                || msg.to_lowercase().contains("administrator")
            {
                "\n  💡 Tip: Run as Administrator"
            } else {
                ""
            }
        )
        .into());
    }

    Ok(stdout)
}

/// Parse "192.168.1.50/24" → ("192.168.1.50", 24)
fn parse_cidr(cidr: &str) -> Result<(String, u8)> {
    let parts: Vec<&str> = cidr.splitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid CIDR '{}'. Use format IP/prefix, e.g. 192.168.1.50/24",
            cidr
        )
        .into());
    }
    let ip = parts[0].to_string();
    let prefix: u8 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid prefix length '{}' in '{}'", parts[1], cidr))?;
    if prefix > 32 {
        return Err(format!("Prefix length {} out of range (0–32)", prefix).into());
    }
    // Basic IPv4 validation
    let octets: Vec<&str> = ip.split('.').collect();
    if octets.len() != 4 || octets.iter().any(|o| o.parse::<u8>().is_err()) {
        return Err(format!("Invalid IPv4 address '{}'", ip).into());
    }
    Ok((ip, prefix))
}

/// Convert prefix length to dotted-decimal netmask
fn prefix_to_mask(prefix: u8) -> String {
    use std::net::Ipv4Addr;
    let mask: u32 = if prefix == 0 {
        0
    } else if prefix >= 32 {
        0xFFFF_FFFF
    } else {
        !((1u32 << (32 - prefix)) - 1)
    };
    Ipv4Addr::from(mask).to_string()
}
