use axum::http::StatusCode;

use super::reponse::{EmptyData, Res};

pub type AppResult<T = ()> = std::result::Result<T, AppErr>;

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("{} not found", 0)]
    NotFoundErr(String),
    #[error("{}", 0)]
    UnauthorizedErr(String),
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

#[test]
fn app_err_test() {
    use redis::RedisError;
    use std::io;
    let redis_err = RedisError::from(io::Error::other("oh no"));
    let app_err = AppErr::from(redis_err).into_res();
    println!("app err {app_err:?}");
    assert_eq!(1, 2)
}
