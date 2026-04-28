// win-ifconfig — A colorful, feature-rich ifconfig for Windows
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Home:   https://github.com/cumulus13/win-ifconfig
// License: MIT

#![allow(non_snake_case)]

mod display;
mod network;
mod types;

use clap::{ArgAction, Parser};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "ifconfig",
    version = env!("CARGO_PKG_VERSION"),
    author = "Hadi Cahyadi <cumulus13@gmail.com>",
    about = "🌐 win-ifconfig — Linux-style ifconfig for Windows with colors, metrics & more",
    long_about = r#"
╔══════════════════════════════════════════════════════════╗
║           win-ifconfig v1.0.5  by Hadi Cahyadi           ║
║     Linux-compatible network interface information       ║
║     with Windows extras: metrics, DNS, routing & more    ║
╚══════════════════════════════════════════════════════════╝

Display network interface information in Linux ifconfig style
with extended Windows-specific details including:
  • Interface metrics and routing priorities
  • DNS servers per adapter
  • DHCP lease information
  • Interface statistics (packets, errors, drops)
  • MTU, speed, and duplex information
  • IPv6 addresses and link-local
  • MAC address with vendor lookup hints
"#,
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

    /// Show verbose/debug information
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Watch mode: refresh every N seconds
    #[arg(short, long, value_name = "SECONDS")]
    pub watch: Option<u64>,

    /// Show summary statistics at the end
    #[arg(long, action = ArgAction::SetTrue)]
    pub summary: bool,
}

fn main() {
    let cli = Cli::parse();

    // Disable colors if requested or not a terminal
    if cli.no_color {
        colored::control::set_override(false);
    }

    if let Some(interval) = cli.watch {
        run_watch_mode(&cli, interval);
    } else {
        if let Err(e) = run_once(&cli) {
            eprintln!("❌ Error: {}", e);
            process::exit(1);
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
