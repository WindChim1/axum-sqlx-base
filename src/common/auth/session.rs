use std::sync::Arc;

use tracing::info;
use uuid::Uuid;

use crate::common::{
    claim::UserClaims,
    client::redis::{RedisClient, RedisClientExt},
    constant::SESSION_KEY_PREFIX,
    error::{AppErr, AppResult},
};

use super::key::{Key, SessionKey};

pub struct Session {
    pub redis_cli: Arc<RedisClient>,
}

impl Session {
    pub fn new(redis_cli: Arc<RedisClient>) -> Self {
        Self { redis_cli }
    }

    pub async fn check(&self, claim: &UserClaims) -> AppResult<bool> {
        let session_key = SessionKey {
            user_id: format!("{}{}", SESSION_KEY_PREFIX, claim.uid),
        };

        //检查session是否存在
        let session_id = self
            .redis_cli
            .get(&session_key.to_string())
            .await?
            .ok_or_else(|| AppErr::NotFoundErr(format!("redis client:{}", session_key)))?;
        //检查session_id是否一致
        if session_id != claim.sid.to_string() {
            info!("Session id invalid so deleting it:{:?}", session_key);
            self.redis_cli.del(&session_key.to_string()).await?;
            return Err(AppErr::InvalidSessionErr("Session is Invalid".to_string()));
        }
        Ok(true)
    }

    pub async fn set(&self, user_id: i32) -> AppResult<Uuid> {
        info!("Generating session for user_id:{:?}", user_id);

        let (session_key, session_value) = Self::generate(user_id);
        self.redis_cli
            .set(
                &session_key.to_string(),
                &session_value.to_string(),
                session_key.expir(),
            )
            .await?;
        Ok(session_value)
    }
    //生成session_key和session_id
    fn generate(user_id: i32) -> (SessionKey, Uuid) {
        let key = SessionKey {
            user_id: format!("{SESSION_KEY_PREFIX}{user_id}"),
        };
        let session_id = key.value();
        (key, session_id)
    }
}
