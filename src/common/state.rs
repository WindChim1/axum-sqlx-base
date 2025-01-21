use std::sync::Arc;

use super::{
    client::{database::DatabaseClient, redis::RedisClient, ClientBuilder},
    config::AppConfig,
    error::AppResult,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
}

impl AppState {
    pub async fn new(app_config: AppConfig) -> AppResult<Self> {
        let database_client = DatabaseClient::build_from_config(&app_config).await?;
        let redis_client = RedisClient::build_from_config(&app_config).await?;
        Ok(Self {
            config: Arc::new(app_config),
            redis: Arc::new(redis_client),
            db: Arc::new(database_client),
        })
    }
}
