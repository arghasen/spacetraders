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

/// SupplyLevel : The supply level of a trade good.
/// The supply level of a trade good.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupplyLevel {
    #[serde(rename = "SCARCE")]
    Scarce,
    #[serde(rename = "LIMITED")]
    Limited,
    #[serde(rename = "MODERATE")]
    Moderate,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "ABUNDANT")]
    Abundant,

}

impl std::fmt::Display for SupplyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Scarce => write!(f, "SCARCE"),
            Self::Limited => write!(f, "LIMITED"),
            Self::Moderate => write!(f, "MODERATE"),
            Self::High => write!(f, "HIGH"),
            Self::Abundant => write!(f, "ABUNDANT"),
        }
    }
}

impl Default for SupplyLevel {
    fn default() -> SupplyLevel {
        Self::Scarce
    }
}

