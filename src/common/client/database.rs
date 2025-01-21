use tracing::info;

use crate::common::{config::database::DatabaseConfig, error::AppResult};

use super::ClientBuilder;

pub type DatabaseClient = sqlx::MySqlPool;

impl ClientBuilder for DatabaseClient {
    async fn build_from_config(config: &crate::common::config::AppConfig) -> AppResult<Self> {
        let DatabaseConfig {
            max_connections, ..
        } = config.db;

        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(&config.db.get_url())
            .await?;

        info!("Database connection");
        Ok(pool)
    }
}

#[tokio::test]
async fn database_client_conn_test() -> AppResult<()> {
    dotenvy::dotenv().ok();
    use crate::common::config::env;
    use crate::common::config::AppConfig;

    let config = AppConfig::inint_config(env::get_env_source("APP")).unwrap();

    println!("config {:?}", config);
    let client = DatabaseClient::build_from_config(&config).await?;
    let query = sqlx::query("select 1").fetch_one(&client).await?;
    println!("query {:?}", query);

    assert_eq!(1, 2);
    Ok(())
}
