use std::time::Duration;

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    error::{AppErr, AppResult},
    role::Role,
};

pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));

pub static DECODE_HEADER: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserClaims {
    //签发时间
    pub iat: i64,
    //过期时间
    pub exp: i64,
    //user id
    pub uid: Uuid,
    //session id
    pub sid: Uuid,
    //role user
    pub rol: Role,
}

impl UserClaims {
    pub fn new(duration: Duration, user_id: Uuid, session_id: Uuid, role: Role) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: user_id,
            sid: session_id,
            rol: role,
        }
    }

    //加密
    pub fn encode(&self, key: &EncodingKey) -> AppResult<String> {
        let token = jsonwebtoken::encode(&ENCODE_HEADER, self, key)?;
        Ok(token)
    }
    //解密
    pub fn decode(token: &str, key: &DecodingKey) -> AppResult<TokenData<Self>> {
        Ok(jsonwebtoken::decode(token, key, &DECODE_HEADER)?)
    }
}

pub trait UserClaimsRequest {
    fn get_user_claims(&self) -> AppResult<UserClaims>;
    fn get_user_id(&self) -> AppResult<Uuid>;
}

impl UserClaimsRequest for axum::extract::Request {
    fn get_user_claims(&self) -> AppResult<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| AppErr::UnauthorizedErr("User must login".to_string()))
    }

    fn get_user_id(&self) -> AppResult<Uuid> {
        self.extensions()
            .get::<UserClaims>()
            .map(|uc| uc.uid)
            .ok_or_else(|| AppErr::UnauthorizedErr("User must login".to_string()))
    }
}

#[test]
fn user_claims_test() {
    use super::constant::ACCESS_TOKEN_DECODE_KEY;
    use super::constant::ACCESS_TOKEN_ENCODE_KEY;
    let user_claims = UserClaims::new(
        Duration::from_secs(60),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Role::default(),
    );

    let token = user_claims.encode(&ACCESS_TOKEN_ENCODE_KEY).unwrap();
    println!("token ={}", token);
    let TokenData { claims, .. } = UserClaims::decode(&token, &ACCESS_TOKEN_DECODE_KEY).unwrap();
    assert_eq!(claims, user_claims)
}
