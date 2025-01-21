use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use tracing::error;

#[derive(Serialize, Debug, Default)]
pub struct Res<T> {
    pub code: u16,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> axum::response::Response {
        let json_string = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize reponse:{}", e);
                serde_json::json!({
                    "code":500,
                    "data":null,
                    "message":"Internal server error"
                })
                .to_string()
            }
        };

        axum::response::Response::builder()
            .status(StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_string))
            .unwrap()
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_success() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            message: Some("Success".to_string()),
        }
    }

    pub fn with_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            message: Some("Success".to_string()),
        }
    }

    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            message: Some(msg.to_string()),
        }
    }

    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            message: Some(msg.to_string()),
        }
    }

    pub fn with_err_msg(msg: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            data: None,
            message: Some(msg.to_string()),
        }
    }
    pub fn with_err_code_msg(code: u16, msg: &str) -> Self {
        Self {
            code,
            data: None,
            message: Some(msg.to_string()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct EmptyData;

impl Res<EmptyData> {
    pub fn with_not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND.as_u16(),
            data: None,
            message: Some(StatusCode::NOT_FOUND.to_string()),
        }
    }
}
