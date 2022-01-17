
use actix_web::{HttpRequest, web};
use pvrustlib::{ServiceError};

use crate::{schema::{V1InferTagRequest, V1InferTagResponse}, autotag::*, SERVICE_NAME};

pub async fn v1_infer_tag(_request: HttpRequest, body: actix_web::web::Json<V1InferTagRequest>) -> Result<web::Json<V1InferTagResponse>, ServiceError> {
	let ret = v1_infer_tag_impl(body.0).await;
	match ret {
		Ok(result) => Ok(web::Json(result)),
		Err(e) => Err(ServiceError::from_dyn_error(SERVICE_NAME, e)),
	}
}
