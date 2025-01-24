use std::future::Future;

use axum::response::IntoResponse;
use common::reponse::{EmptyData, Res};
use tokio::signal;

pub mod common;
pub mod dto;
pub mod entity;
pub mod handlers;
pub mod repository;
pub mod routers;
pub mod services;
pub mod utils;
pub mod vo;

pub async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
}

pub async fn shutdown_signal() -> impl Future<Output = ()> {
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
