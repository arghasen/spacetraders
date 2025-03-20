mod client;

use anyhow::Result;
use client::SpaceTradersClient;
use colored::*;
use console::Term;
use dotenv::dotenv;
use log::{error, info};
use std::time::Duration;

fn print_banner() {
    println!("\n{}", "🚀 Space Traders API Client".bright_cyan().bold());
    println!("{}", "=========================".bright_cyan());
}

async fn startup_sequence() -> Result<String> {
    let term = Term::stdout();
    term.clear_screen()?;
    print_banner();

    // Initialize environment
    info!("Loading environment variables...");
    dotenv().ok();
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Initialize logging
    info!("Initializing logging system...");
    env_logger::init();
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Load API token
    info!("Loading API credentials...");
    let api_token = match std::env::var("SPACE_TRADERS_API_TOKEN") {
        Ok(token) => {
            info!("✅ System initialized successfully!");
            token
        }
        Err(_) => {
            error!("❌ Failed to load API token!");
            anyhow::bail!("SPACE_TRADERS_API_TOKEN must be set in .env file");
        }
    };

    println!("\n{}", "System Status:".yellow().bold());
    println!("├─ {} {}", "Environment:".blue(), "✓".green());
    println!("├─ {} {}", "Logging:".blue(), "✓".green());
    println!("└─ {} {}", "API Token:".blue(), "✓".green());
    println!();

    Ok(api_token)
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_token = startup_sequence().await?;
    info!("Space Traders API client ready for commands!");

    let client = SpaceTradersClient::new(api_token);

    Ok(())
}
