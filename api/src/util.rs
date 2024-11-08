use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub trait AnyhowError {
    fn anyhow_resp(self) -> Response;
}

impl<T> AnyhowError for anyhow::Result<T>
where
    T: Serialize,
{
    fn anyhow_resp(self) -> Response {
        match self {
            Ok(b) => Json(b).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

pub trait AnyhowRspc<T> {
    fn anyhow_rspc(self) -> Result<T, rspc::Error>;
}

impl<T> AnyhowRspc<T> for anyhow::Result<T>
where
    T: Serialize,
{
    fn anyhow_rspc(self) -> Result<T, rspc::Error> {
        self.map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string()))
    }
}
