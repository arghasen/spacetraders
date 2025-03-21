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

/// Contract : Contract details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// ID of the contract.
    #[serde(rename = "id")]
    pub id: String,
    /// The symbol of the faction that this contract is for.
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    /// Type of contract.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "terms")]
    pub terms: Box<models::ContractTerms>,
    /// Whether the contract has been accepted by the agent
    #[serde(rename = "accepted")]
    pub accepted: bool,
    /// Whether the contract has been fulfilled
    #[serde(rename = "fulfilled")]
    pub fulfilled: bool,
    /// Deprecated in favor of deadlineToAccept
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// The time at which the contract is no longer available to be accepted
    #[serde(rename = "deadlineToAccept", skip_serializing_if = "Option::is_none")]
    pub deadline_to_accept: Option<String>,
}

impl Contract {
    /// Contract details.
    pub fn new(id: String, faction_symbol: String, r#type: Type, terms: models::ContractTerms, accepted: bool, fulfilled: bool, expiration: String) -> Contract {
        Contract {
            id,
            faction_symbol,
            r#type,
            terms: Box::new(terms),
            accepted,
            fulfilled,
            expiration,
            deadline_to_accept: None,
        }
    }
}
/// Type of contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PROCUREMENT")]
    Procurement,
    #[serde(rename = "TRANSPORT")]
    Transport,
    #[serde(rename = "SHUTTLE")]
    Shuttle,
}

impl Default for Type {
    fn default() -> Type {
        Self::Procurement
    }
}

