use actix_web::{error::InternalError, http::StatusCode, HttpResponse, Responder};
use serde::Serialize;

pub trait ToResponder {
    fn to_responder(self) -> impl Responder;
}

impl<T> ToResponder for anyhow::Result<T>
where
    T: Serialize,
{
    fn to_responder(self) -> impl Responder {
        self.map(|b| HttpResponse::Ok().json(b))
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
    }
}
