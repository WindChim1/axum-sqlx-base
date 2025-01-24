use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::common::constant::ACCESS_TOKEN_DECODE_KEY;

use super::{claim::UserClaims, error::AppErr, state::AppState};

//从header中提取claims
impl FromRequestParts<AppState> for UserClaims {
    type Rejection = AppErr;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;
        let user_claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)?.claims;

        state.session.check(&user_claims).await?;
        Ok(user_claims)
    }
}
