use ::tracing::info;
use axum_sqlx_base::{
    common::{config::tracing, constant::APP_CONFIG, state::AppState},
    handler_404,
    routers::set_routers,
    shutdown_signal,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //加载.env  环境配置文件
    dotenvy::dotenv().ok();
    //初始化日志
    let file_appender_guard = tracing::LogGuard::init_logs().await;
    info!("The initialization of Tracing was successful");
    //初始化配置
    let app_config = APP_CONFIG.as_ref().expect("Config init  error");
    //加载appState
    let app_state = AppState::new(app_config.clone()).await?;
    //设置路由
    let app = set_routers(app_state).fallback(handler_404);
    //地址绑定
    let listener = tokio::net::TcpListener::bind(app_config.server.get_socket_addr()?).await?;
    info!("🚀 lisening on {}", &listener.local_addr()?);

    //启动axum服务
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal().await)
        .await
        .unwrap();

    drop(file_appender_guard);
    Ok(())
}
