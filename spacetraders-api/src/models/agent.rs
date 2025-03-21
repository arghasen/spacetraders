/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.3.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Agent : Agent details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    /// Account ID that is tied to this agent. Only included on your own agent.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Symbol of the agent.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The headquarters of the agent.
    #[serde(rename = "headquarters")]
    pub headquarters: String,
    /// The number of credits the agent has available. Credits can be negative if funds have been overdrawn.
    #[serde(rename = "credits")]
    pub credits: i64,
    /// The faction the agent started with.
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
    /// How many ships are owned by the agent.
    #[serde(rename = "shipCount")]
    pub ship_count: i32,
}

impl Agent {
    /// Agent details.
    pub fn new(symbol: String, headquarters: String, credits: i64, starting_faction: String, ship_count: i32) -> Agent {
        Agent {
            account_id: None,
            symbol,
            headquarters,
            credits,
            starting_faction,
            ship_count,
        }
    }
}

