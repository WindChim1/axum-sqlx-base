pub mod database;
pub mod env;
pub mod profile;
pub mod redis;
pub mod server;
pub mod tracing;

use ::tracing::info;
use config::{ConfigError, Environment, File};
use database::DatabaseConfig;
use env::get_profile;
use profile::Profile;
use redis::RedisConfig;
use serde::Deserialize;
use server::ServerConfig;
use tracing::TracingConfig;

use crate::utils::dir::get_project_root;

use super::error::AppResult;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AppConfig {
    pub profile: Profile,
    pub tracing: TracingConfig,
    pub server: ServerConfig,
    pub db: DatabaseConfig,
    pub redis: RedisConfig,
}

impl AppConfig {
    pub fn inint_config(env_src: Environment) -> AppResult<Self> {
        //获取配置文件目录
        let config_dir = get_settings_dir()?;
        //获取配置文件环境
        let run_mode = get_profile()?;
        //当前配置文件
        let profile_filename = format!("{}.toml", run_mode);
        let config = config::Config::builder()
            //添加默认配置
            .add_source(File::from(config_dir.join("default.toml")))
            //添加自定义前缀配置
            .add_source(File::from(config_dir.join(profile_filename)))
            //添加环境变量
            .add_source(env_src)
            .build()?;
        info!("Successfully read config profile: {run_mode}.");
        Ok(config.try_deserialize()?)
    }
}

pub fn get_settings_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

#[test]
fn display_test() {
    let run_mode = Profile::Dev;
    let profile_filename = format!("{}.toml", run_mode);
    println!("{profile_filename}");
    assert_eq!(1, 2)
}
#[test]
pub fn test_read_app_config_prefix() {
    use env::get_env_source;
    dotenvy::dotenv().ok();
    // 读取配置
    let config = AppConfig::inint_config(get_env_source("APP")).unwrap();
    println!("config: {:#?}", config);

    assert_eq!(1, 2)
}
