use std::str::FromStr;

use config::ConfigError;

use super::profile::Profile;

pub fn get_env_source(prefix: &str) -> config::Environment {
    config::Environment::with_prefix(prefix)
        .prefix_separator("_")
        .separator("_")
}

pub fn get_profile() -> Result<Profile, config::ConfigError> {
    std::env::var("RUN_MODE")
        .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
        .unwrap_or(Ok(Profile::Dev))
}

#[test]
fn get_env_source_test() {
    dotenvy::dotenv().ok();
    let env = get_env_source("APP");
    println!("env:{:?}", env);
    assert_eq!(1, 2)
}
