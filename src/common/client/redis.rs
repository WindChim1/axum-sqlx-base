use std::time::Duration;

use tracing::info;

use crate::common::error::AppResult;

use super::ClientBuilder;

pub type RedisClient = redis::Client;

pub trait RedisClientExt: ClientBuilder {
    fn ping(&self) -> impl std::future::Future<Output = AppResult<Option<String>>>;
    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl std::future::Future<Output = AppResult<()>>;
    fn get(&self, key: &str) -> impl std::future::Future<Output = AppResult<String>>;
    fn exist(&self, key: &str) -> impl std::future::Future<Output = AppResult<bool>>;
    fn del(&self, key: &str) -> impl std::future::Future<Output = AppResult<bool>>;
    fn ttl(&self, key: &str) -> impl std::future::Future<Output = AppResult<i64>>;
}

impl ClientBuilder for RedisClient {
    async fn build_from_config(config: &crate::common::config::AppConfig) -> AppResult<Self> {
        Ok(redis::Client::open(config.redis.get_url())?)
    }
}

impl RedisClientExt for RedisClient {
    async fn ping(&self) -> AppResult<Option<String>> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("PING").query_async(&mut conn).await?;
        info!("ping redis serve");
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> AppResult<()> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async::<String>(&mut conn)
            .await?;
        info!("redis set key: {}", msg);
        let msg = redis::cmd("EXPIRE")
            .arg(key)
            .arg(expire.as_secs().to_string())
            .query_async::<i32>(&mut conn)
            .await?;
        info!("redis set expire time:{}", msg);
        Ok(())
    }

    async fn get(&self, key: &str) -> AppResult<String> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("GET")
            .arg(key)
            .query_async::<String>(&mut conn)
            .await?;
        info!("redis get value:{}", value);
        Ok(value)
    }

    async fn exist(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        info!("redis check key exists:{}", key);
        Ok(msg)
    }

    async fn del(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        info!("redis delete key:{}", key);
        Ok(msg)
    }

    async fn ttl(&self, key: &str) -> AppResult<i64> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        info!("redis TTL key:{}", key);
        Ok(msg)
    }
}

#[tokio::test]
async fn redis_client_test() -> AppResult<()> {
    use crate::common::config::{env::get_env_source, AppConfig};
    let config = AppConfig::inint_config(get_env_source("APP")).unwrap();
    let redis_client = RedisClient::build_from_config(&config).await?;
    let ping_result = redis_client.ping().await?.unwrap();
    let exit = redis_client.exist("wdc").await?;
    assert_eq!(ping_result, "PONG");
    assert!(!exit);
    Ok(())
}

#[cfg(test)]
mod redis_test_mod {

    use std::time::Duration;

    use crate::common::{
        client::{redis::RedisClientExt, ClientBuilder},
        config::{env::get_env_source, AppConfig},
        error::AppResult,
    };

    use super::RedisClient;

    pub async fn build_redis_client() -> AppResult<RedisClient> {
        let app_config = AppConfig::inint_config(get_env_source("APP")).unwrap();
        RedisClient::build_from_config(&app_config).await
    }

    #[tokio::test]
    async fn redis_client_ping_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.ping().await?.unwrap();
        assert_eq!(result, "PONG");
        Ok(())
    }

    #[tokio::test]
    async fn redis_client_set_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        redis_client
            .set("name", "wdc", Duration::from_secs(600))
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn redis_client_get_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.get("name").await?;
        assert_eq!(result, "wdc");
        Ok(())
    }
    #[tokio::test]
    async fn redis_client_exists_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.exist("name").await?;
        assert!(result);
        Ok(())
    }
    #[tokio::test]
    async fn redis_client_ttl_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.ttl("name").await?;
        assert!(result > 0);
        Ok(())
    }

    #[tokio::test]
    async fn redis_client_del_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.del("name").await?;
        assert!(result);
        Ok(())
    }

    #[tokio::test]
    async fn redis_client_exists2_test() -> AppResult<()> {
        let redis_client: RedisClient = build_redis_client().await?;
        let result = redis_client.exist("name").await?;
        assert!(!result);
        Ok(())
    }
}
