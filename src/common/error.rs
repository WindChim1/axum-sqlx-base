use axum::{http::StatusCode, response::IntoResponse};

use super::reponse::{EmptyData, Res};

pub type AppResult<T = ()> = std::result::Result<T, AppErr>;

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("{0} not found")]
    NotFoundErr(String),
    #[error("{0}")]
    UserNotFoundErr(String),
    #[error("{0}")]
    UnauthorizedErr(String),
    #[error("{0}")]
    InvalidSessionErr(String),
    #[error("{0}")]
    Argon2Err(String),
    #[error(transparent)]
    RedisErr(#[from] redis::RedisError),
    #[error(transparent)]
    SqlxErr(#[from] sqlx::Error),
    #[error(transparent)]
    JwtErr(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    ConfigErr(#[from] config::ConfigError),
    #[error(transparent)]
    ParseErr(#[from] std::net::AddrParseError),
    #[error(transparent)]
    TypeHeaderErr(#[from] axum_extra::typed_header::TypedHeaderRejection),
    #[error(transparent)]
    SpawnTaskErr(#[from] tokio::task::JoinError),
}
//手动实现from trait，可将argon2 err 通过？转换给 AppErr
impl From<argon2::password_hash::Error> for AppErr {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppErr::Argon2Err(value.to_string())
    }
}

impl AppErr {
    pub fn into_res(self) -> Res<EmptyData> {
        let msg = self.to_string();
        let code = match self {
            AppErr::NotFoundErr(_resource) => StatusCode::NOT_FOUND.as_u16(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        };
        Res::with_err_code_msg(code, &msg)
    }
}

//实现IntoResponse trait, 当handler返回AppErr时可以正常返回给前端
impl IntoResponse for AppErr {
    fn into_response(self) -> axum::response::Response {
        self.into_res().into_response()
    }
}

#[test]
fn app_err_test() {
    use redis::RedisError;
    use std::io;
    let redis_err = RedisError::from(io::Error::other("oh no"));
    let app_err = AppErr::from(redis_err).into_res();
    println!("app err {app_err:?}");
    assert_eq!(1, 2)
}

#[test]
fn app_err_into_resp_test() {
    let not_found_err = AppErr::NotFoundErr("redis".to_string());
    let resp = not_found_err.into_res();
    let resp = resp.into_response();
    println!("resp:{:?}", resp);
    assert_eq!(1, 2)
}
