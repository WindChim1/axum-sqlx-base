use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};

use crate::common::error::AppResult;
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    // 获取地址
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    // 获取http地址
    pub fn get_http_addr(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
    // 获取socket地址
    pub fn get_socket_addr(&self) -> AppResult<SocketAddr> {
        let addr = self.get_addr().parse()?;
        Ok(addr)
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn app_config_http_addr_test() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(config.get_http_addr(), "http://127.0.0.1:8080");
    }
    #[test]
    pub fn app_config_socket_addr_test() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(
            config.get_socket_addr().unwrap().to_string(),
            "127.0.0.1:8080"
        );
    }
}
