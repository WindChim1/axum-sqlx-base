use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl RedisConfig {
    pub fn get_url(&self) -> String {
        format!(
            "redis://{username}:{password}@{host}:{port}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
        )
    }
}
