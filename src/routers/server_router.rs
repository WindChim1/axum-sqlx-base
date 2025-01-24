use axum::routing::get;

use crate::common::{reponse::Res, state::AppState};

pub fn set_up_server_router(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.route(
        "/server/health_check",
        get(|| async { Res::<String>::with_msg("Server is up running") }),
    )
}
