use axum::{extract::State, Json};
use tracing::info;

use crate::{
    common::{error::AppResult, reponse::Res, state::AppState},
    dto::token_dto::TokenDto,
    services::{user_service::UserService, FromAppState},
    vo::user_vo::UserVo,
};

pub async fn sign_in(
    State(state): State<AppState>,
    Json(user_vo): Json<UserVo>,
) -> AppResult<Res<TokenDto>> {
    let login_name = &user_vo.login_name.clone();
    info!("{} Sign in ", login_name);
    let user_service = UserService::from_state(&state);
    let token = user_service.sign_in(user_vo).await?;
    info!("{} Sign in success", login_name);
    Ok(Res::with_data(token))
}
