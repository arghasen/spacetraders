use anyhow::Result;
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

pub struct SpaceTradersClient {
    pub client: Client,
    pub token: String,
}

impl SpaceTradersClient {
    pub fn new(token: String) -> Self {
        let client = Client::new();
        Self { client, token }
    }

    pub async fn get_agent(&self) -> Result<Agent> {
        let response = self
            .client
            .get("https://api.spacetraders.io/v2/agent")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        let api_response: ApiResponse<Agent> = response.json().await?;
        println!("✅ Agent data retrieved successfully!");

        Ok(api_response.data)
    }

    pub async fn get_status(&self) -> Result<()> {
        let response = self
            .client
            .get("https://api.spacetraders.io/v2/")
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ API is operational!");
        } else {
            println!("❌ API is not responding!");
        }

        Ok(())
    }
}
