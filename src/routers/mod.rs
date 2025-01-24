pub mod server_router;
pub mod token_router;
pub mod user_router;
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

impl SetupRouter for Router<AppState> {}

pub fn set_routers(state: AppState) -> Router {
    let nest = Router::new()
        .setup_router(server_router::set_up_server_router)
        .setup_router(token_router::setup_token_routers)
        .setup_router(user_router::setup_user_routers);
    Router::new().nest("/api", nest).with_state(state)
}
