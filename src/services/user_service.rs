use std::sync::Arc;

use crate::{
    common::{
        auth::{session::Session, token::Token},
        client::redis::RedisClient,
        error::AppResult,
        state::{self, AppState},
    },
    dto::token_dto::TokenDto,
    repository::user_repository::UserRepository,
    utils::passwor,
    vo::user_vo::UserVo,
};

use super::FromAppState;

pub struct UserService {
    user_repository: Arc<UserRepository>,
    redis: Arc<RedisClient>,
    session: Arc<Session>,
    token: Arc<Token>,
}

impl FromAppState for UserService {
    fn from_state(state: &AppState) -> Self {
        Self {
            user_repository: Arc::new(UserRepository::new(state.db.clone())),
            redis: state.redis.clone(),
            session: state.session.clone(),
            token: state.token.clone(),
        }
    }
}

impl UserService {
    pub async fn sign_in(&self, user_vo: UserVo) -> AppResult<TokenDto> {
        //检查用户是否存在
        let user = self
            .user_repository
            .find_by_login_name(&user_vo.login_name)
            .await?;
        //校验密码
        passwor::verify(user_vo.password, user.password).await?;
        //登录
        //生成session_key与session_id
        let session_id = self.session.set(user.user_id).await?;
        let token = self
            .token
            .generate_token(user.user_id, user.role, session_id)?;
        Ok(token)
    }
}
