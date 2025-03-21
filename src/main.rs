use anyhow::Result;
use colored::*;
use console::Term;
use dotenv::dotenv;
use log::{error, info};

mod client;
mod ui;

use client::SpaceTradersClient;
use ui::{run_app, App};

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

    // Initialize logging
    info!("Initializing logging system...");
    env_logger::init();

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

    // Create client
    let client = SpaceTradersClient::new(api_token);

    // Create and run the app
    let mut app = App::new(client);
    run_app(&mut app).await?;

    Ok(())
}
