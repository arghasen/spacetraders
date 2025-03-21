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

/// Cooldown : A cooldown is a period of time in which a ship cannot perform certain actions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cooldown {
    /// The symbol of the ship that is on cooldown
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    /// The total duration of the cooldown in seconds
    #[serde(rename = "totalSeconds")]
    pub total_seconds: i32,
    /// The remaining duration of the cooldown in seconds
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds: i32,
    /// The date and time when the cooldown expires in ISO 8601 format
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

impl Cooldown {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub fn new(ship_symbol: String, total_seconds: i32, remaining_seconds: i32) -> Cooldown {
        Cooldown {
            ship_symbol,
            total_seconds,
            remaining_seconds,
            expiration: None,
        }
    }
}

