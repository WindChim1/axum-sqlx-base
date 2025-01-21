use crate::common::{config::AppConfig, error::AppResult};

pub mod database;
pub mod redis;

pub trait ClientBuilder: Sized {
    fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = AppResult<Self>>;
}
