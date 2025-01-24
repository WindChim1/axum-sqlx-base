use crate::common::state::AppState;

pub mod user_service;

pub trait FromAppState: Sized {
    fn from_state(statte: &AppState) -> Self;
}
