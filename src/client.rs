use anyhow::Result;
use spacetraders_api::apis::agents_api;
use spacetraders_api::apis::configuration::Configuration;
use spacetraders_api::apis::fleet_api;
use spacetraders_api::apis::global_api;
use spacetraders_api::apis::systems_api;

pub struct SpaceTradersClient {
    config: Configuration,
}

impl SpaceTradersClient {
    pub fn new(api_token: String) -> Self {
        let mut config = Configuration::new();
        config.bearer_access_token = Some(api_token);
        Self { config }
    }

    pub async fn get_status(&self) -> Result<spacetraders_api::models::GetStatus200Response> {
        Ok(global_api::get_status(&self.config).await?)
    }

    pub async fn get_my_agent(&self) -> Result<spacetraders_api::models::Agent> {
        let response = agents_api::get_my_agent(&self.config).await?;
        Ok(*response.data)
    }

    pub async fn get_my_ships(&self) -> Result<Vec<spacetraders_api::models::Ship>> {
        let response = fleet_api::get_my_ships(&self.config, None, None).await?;
        Ok(response.data)
    }

    pub async fn get_systems(
        &self,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<spacetraders_api::models::System>> {
        let response = systems_api::get_systems(&self.config, page, limit).await?;
        Ok(response.data)
    }
}
