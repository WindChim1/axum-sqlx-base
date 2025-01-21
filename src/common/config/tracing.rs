use serde::Deserialize;
use std::env;

use tracing::dispatcher::set_global_default;
use tracing_appender::rolling::daily;
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    layer::SubscriberExt,
    EnvFilter, Registry,
};
#[derive(Debug, Deserialize, Clone)]
pub struct TracingConfig {
    log_level: String,
}
impl TracingConfig {
    // 获取地址
    pub fn get_log_level(&self) -> String {
        self.log_level.to_string()
    }
}

pub struct LogGuard(pub std::sync::Arc<tracing_appender::non_blocking::WorkerGuard>);
impl LogGuard {
    pub async fn init_logs() -> LogGuard {
        // 读取日志级别
        let log_level = env::var("LOG_LEVEL").unwrap_or("debug".to_string());
        // 设置日志级别过滤器
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(log_level))
            .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into());
        // 创建每日滚动的日志文件写入器
        let file_appender = daily("logs", "app.log");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
        // 初始化日志订阅者，同时输出到控制台和文件
        let subscriber = Registry::default()
            .with(env_filter)
            // 控制台输出层
            .with(fmt::layer().with_ansi(true).with_target(false))
            // 文件输出层
            .with(
                fmt::Layer::new()
                    .with_writer(non_blocking_appender)
                    .with_timer(UtcTime::rfc_3339()),
            );
        // 设置为全局日志订阅者
        set_global_default(subscriber.into()).expect("setting default subscriber failed");
        LogGuard(std::sync::Arc::new(guard))
    }
}
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn app_config_http_addr_test() {
        let config = TracingConfig {
            log_level: "debug".to_string(),
        };
        assert_eq!(config.get_log_level(), "debug");
    }
}
