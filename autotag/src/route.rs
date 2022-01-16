
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, http::StatusCode, web};
use pvrustlib::{EmptyJSON, ServiceError};

use crate::schema::{V1InferTagRequest, V1InferTagResponse};

pub async fn v1_infer_tag(request: HttpRequest, body: actix_web::web::Json<V1InferTagRequest>) -> Result<web::Json<V1InferTagResponse>, ServiceError> {
	todo!()
}
