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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtractResources201ResponseData {
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::Cooldown>,
    #[serde(rename = "extraction")]
    pub extraction: Box<models::Extraction>,
    #[serde(rename = "cargo")]
    pub cargo: Box<models::ShipCargo>,
    #[serde(rename = "events")]
    pub events: Vec<models::ShipConditionEvent>,
}

impl ExtractResources201ResponseData {
    pub fn new(cooldown: models::Cooldown, extraction: models::Extraction, cargo: models::ShipCargo, events: Vec<models::ShipConditionEvent>) -> ExtractResources201ResponseData {
        ExtractResources201ResponseData {
            cooldown: Box::new(cooldown),
            extraction: Box::new(extraction),
            cargo: Box::new(cargo),
            events,
        }
    }
}

