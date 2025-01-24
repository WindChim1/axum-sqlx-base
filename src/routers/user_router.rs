use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    common::{reponse::Res, state::AppState},
    handlers::user_handler::sign_in,
};

pub fn setup_user_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    let nest = Router::new().route("/sign_in", post(sign_in)).route(
        "/health_check",
        get(|| async { Res::<String>::with_msg("user server is up running") }),
    );
    router.nest("/user", nest)
}
