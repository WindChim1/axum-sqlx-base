use axum::routing::get;

use crate::common::{reponse::Res, state::AppState};

pub fn setup_token_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route(
        "/token/health_check",
        get(|| async { Res::<String>::with_msg("Token is up running") }),
    )
}
