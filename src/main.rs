use std::future::Future;

use ::tracing::info;
use axum::response::IntoResponse;
use axum_sqlx_base::{
    common::{
        config::tracing,
        constant::APP_CONFIG,
        reponse::{EmptyData, Res},
        state::AppState,
    },
    routers::set_routers,
};
use tokio::signal;

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
    let app = set_routers(app_state).await.fallback(handler_404);
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

async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
}

async fn shutdown_signal() -> impl Future<Output = ()> {
    async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
        _= ctrl_c =>{
                println!("Ctrl+C signal received.")

            },
        _= terminate =>{
                println!("Terminate signal received.")
            },
            else =>()
        }
    }
}
