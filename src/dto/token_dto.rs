use std::time::Duration;

use serde::Serialize;

use crate::common::constant::TOKEN_TYPE;

#[derive(Serialize, Debug)]
pub struct TokenDto {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: Duration,
}
impl TokenDto {
    pub fn new(access_token: String, refresh_token: String, expire_in: Duration) -> Self {
        Self {
            token_type: TOKEN_TYPE.to_string(),
            access_token,
            refresh_token,
            expire_in,
        }
    }
}
