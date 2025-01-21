pub mod server;
use axum::Router;

use crate::common::state::AppState;

trait SetupRouter {
    fn setup_router<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized,
    {
        f(self)
    }
}

impl SetupRouter for Router {}

pub async fn set_routers(state: AppState) -> Router {
    Router::new()
        .with_state(state)
        .clone()
        .setup_router(server::set_up_server_router)
}
