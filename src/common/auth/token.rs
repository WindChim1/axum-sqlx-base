use uuid::Uuid;

use crate::{
    common::{
        claim::UserClaims,
        constant::{
            ACCESS_TOKEN_ENCODE_KEY, EXPIRE_BEARER_TOKEN_SECS, EXPIRE_REFRESH_TOKEN_SECS,
            REFRESH_TOKEN_ENCODE_KEY,
        },
        error::AppResult,
        role::Role,
    },
    dto::token_dto::TokenDto,
};

pub struct Token;

impl Token {
    pub fn generate_token(
        &self,
        user_id: i32,
        role: Role,
        session_id: Uuid,
    ) -> AppResult<TokenDto> {
        let access_token =
            UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id, role.clone())
                .encode(&ACCESS_TOKEN_ENCODE_KEY)?;

        let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id, role)
            .encode(&REFRESH_TOKEN_ENCODE_KEY)?;

        Ok(TokenDto::new(
            access_token,
            refresh_token,
            EXPIRE_BEARER_TOKEN_SECS,
        ))
    }
}
