use actix_web::{error::InternalError, http::StatusCode, HttpResponse, Responder};
use serde::Serialize;

pub trait ToResponse {
    fn to_response(self) -> HttpResponse;
}

impl<T> ToResponse for anyhow::Result<T>
where
    T: Serialize,
{
    fn to_response(self) -> HttpResponse {
        match self {
            Ok(b) => HttpResponse::Ok().json(b),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}
