// win-ifconfig — A colorful, feature-rich ifconfig for Windows
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Home:   https://github.com/cumulus13/win-ifconfig
// License: MIT

#![allow(non_snake_case)]

mod control;
mod display;
mod network;
mod types;

use clap::{ArgAction, Args, Parser, Subcommand};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "ifconfig",
    version = env!("CARGO_PKG_VERSION"),
    author = "Hadi Cahyadi <cumulus13@gmail.com>",
    about = "🌐 win-ifconfig — Linux-style ifconfig for Windows with colors, metrics & more",
    long_about = r#"
╔══════════════════════════════════════════════════════════╗
║           win-ifconfig v1.0.11  by Hadi Cahyadi           ║
║     Linux-compatible network interface information       ║
║     with Windows extras: metrics, DNS, routing & more    ║
╚══════════════════════════════════════════════════════════╝

READ  (no subcommand):
  ifconfig                        Show active interfaces
  ifconfig -a                     Show all interfaces
  ifconfig Wi-Fi                  Show specific interface
  ifconfig -v                     Verbose (stats + metrics + dns + dhcp)
  ifconfig -j                     JSON output
  ifconfig --watch 2              Auto-refresh every 2s

WRITE (subcommands — require Administrator):
  ifconfig set Wi-Fi ip 192.168.1.50/24 --gw 192.168.1.1
  ifconfig set Wi-Fi ip dhcp
  ifconfig set Ethernet mtu 9000
  ifconfig set Wi-Fi metric 10
  ifconfig set Wi-Fi dns 8.8.8.8 8.8.4.4
  ifconfig set Wi-Fi dns dhcp
  ifconfig up Ethernet
  ifconfig down Ethernet
  ifconfig set Ethernet mac AA:BB:CC:DD:EE:FF
  ifconfig set Ethernet add 10.0.0.50/24
  ifconfig set Ethernet del 10.0.0.50
  ifconfig set Ethernet flush

NOTE: Write operations call netsh/PowerShell and require elevation.
"#,
    subcommand_negates_reqs = true,
)]
pub struct Cli {
    /// Interface name or index to display (show all if omitted)
    #[arg(value_name = "INTERFACE")]
    pub interface: Option<String>,

    /// Show all interfaces including loopback and disconnected
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub all: bool,

    /// Output in JSON format
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub json: bool,

    /// Show extended statistics (packets, errors, drops, collisions)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub stats: bool,

    /// Show routing metrics and gateway information
    #[arg(short = 'm', long, action = ArgAction::SetTrue)]
    pub metrics: bool,

    /// Show DNS configuration per interface
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub dns: bool,

    /// Show DHCP lease information
    #[arg(long, action = ArgAction::SetTrue)]
    pub dhcp: bool,

    /// Disable colored output
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_color: bool,

    /// Compact output (less verbose)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub brief: bool,

    /// Show verbose/debug information (stats + metrics + dns + dhcp)
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Watch/refresh mode: update every N seconds
    #[arg(short, long, value_name = "SECONDS")]
    pub watch: Option<u64>,

    /// Show summary totals at the end
    #[arg(long, action = ArgAction::SetTrue)]
    pub summary: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bring an interface UP  (requires Administrator)
    Up {
        /// Interface name (e.g. "Ethernet", "Wi-Fi")
        interface: String,
    },
    /// Bring an interface DOWN  (requires Administrator)
    Down {
        /// Interface name (e.g. "Ethernet", "Wi-Fi")
        interface: String,
    },
    /// Configure interface properties  (requires Administrator)
    Set {
        /// Interface name (e.g. "Ethernet", "Wi-Fi")
        interface: String,

        #[command(subcommand)]
        operation: SetOperation,
    },
}

#[derive(Subcommand, Debug)]
pub enum SetOperation {
    /// Set IPv4 address (CIDR) or "dhcp"
    #[command(name = "ip")]
    Ip(IpArgs),

    /// Set the MTU in bytes
    Mtu {
        /// MTU value (e.g. 1500, 9000)
        value: u32,
    },

    /// Set the interface routing metric (lower = higher priority)
    Metric {
        /// Metric value
        value: u32,
    },

    /// Set DNS servers, or "dhcp" for automatic
    #[command(name = "dns")]
    Dns(DnsArgs),

    /// Add an additional IPv4 address (CIDR notation)
    Add {
        /// e.g. 10.0.0.50/24
        address: String,
    },

    /// Remove an IPv4 address from the interface
    Del {
        /// e.g. 10.0.0.50
        address: String,
    },

    /// Change the MAC address
    Mac {
        /// e.g. AA:BB:CC:DD:EE:FF
        address: String,
    },

    /// Flush all IPs and reset to DHCP
    Flush,
}

#[derive(Args, Debug)]
pub struct IpArgs {
    /// CIDR address (192.168.1.50/24) or "dhcp"
    pub address: String,

    /// Default gateway
    #[arg(long = "gw", value_name = "GATEWAY")]
    pub gateway: Option<String>,
}

#[derive(Args, Debug)]
pub struct DnsArgs {
    /// DNS servers or "dhcp"
    #[arg(required = true, num_args = 1..)]
    pub servers: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    // Disable colors if requested or not a terminal
    if cli.no_color {
        colored::control::set_override(false);
    }

    match &cli.command {
        Some(Commands::Up { interface }) => {
            if let Err(e) = control::interface_up(interface) {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Down { interface }) => {
            if let Err(e) = control::interface_down(interface) {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Set {
            interface,
            operation,
        }) => {
            if let Err(e) = control::run_set(interface, operation) {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
        None => {
            if let Some(interval) = cli.watch {
                run_watch_mode(&cli, interval);
            } else if let Err(e) = run_once(&cli) {
                eprintln!("❌ Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn run_once(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let adapters = network::get_adapters()?;
    if cli.json {
        display::output_json(&adapters, cli)?;
    } else {
        display::output_text(&adapters, cli)?;
    }
    Ok(())
}

fn run_watch_mode(cli: &Cli, interval: u64) {
    use std::thread;
    use std::time::Duration;

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        let now = chrono::Local::now();
        println!(
            "🔄 Refreshing every {}s — Last update: {}",
            interval,
            now.format("%H:%M:%S")
        );
        println!();
        if let Err(e) = run_once(cli) {
            eprintln!("❌ Error: {}", e);
        }
        thread::sleep(Duration::from_secs(interval));
    }
}
