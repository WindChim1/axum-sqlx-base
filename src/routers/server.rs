use axum::routing::get;

use crate::common::reponse::Res;

pub fn set_up_server_router(router: axum::Router) -> axum::Router {
    router.route(
        "/api/server/health_check",
        get(|| async { Res::<String>::with_msg("Server is up running") }),
    )
}
