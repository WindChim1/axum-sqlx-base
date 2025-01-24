use std::sync::Arc;

use super::{
    auth::{session::Session, token::Token},
    client::{database::DatabaseClient, redis::RedisClient, ClientBuilder},
    config::AppConfig,
    error::AppResult,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub redis: Arc<RedisClient>,
    pub db: Arc<DatabaseClient>,
    pub token: Arc<Token>,
    pub session: Arc<Session>,
}

impl AppState {
    pub async fn new(app_config: AppConfig) -> AppResult<Self> {
        let database_client = DatabaseClient::build_from_config(&app_config).await?;
        let redis_client = Arc::new(RedisClient::build_from_config(&app_config).await?);
        let session = Session::new(redis_client.clone());
        Ok(Self {
            config: Arc::new(app_config),
            redis: redis_client,
            db: Arc::new(database_client),
            token: Arc::new(Token),
            session: Arc::new(session),
        })
    }
}
