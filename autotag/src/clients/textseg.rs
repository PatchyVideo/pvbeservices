use pvrustlib::{ServiceError, ServiceErrorResponse};
use serde::{Serialize, Deserialize};

use crate::SERVICE_NAME;

#[cfg(debug_assertions)]
pub const TEXTSEG_ADDRESS: &str = "http://192.168.0.54:5005";

#[cfg(not(debug_assertions))]
pub const TEXTSEG_ADDRESS: &str = "http://textseg:5005";

#[derive(Deserialize)]
struct CutForSearchResponse {
    #[serde(rename = "Words")]
	pub words: Vec<String>
}

#[derive(Deserialize)]
struct CutForIndexResponse {
    #[serde(rename = "Words")]
	pub words: Vec<String>
}

pub async fn cut_for_search(text: String) -> Result<Vec<String>, ServiceError> {
	let url = format!("{}/s", TEXTSEG_ADDRESS);
	let client = reqwest::Client::new();
	let response = client.post(url.clone())
		.body(text)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(e) => { return Err(ServiceError::new_network_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
	};
	let status = response.status();
	if status.is_success() {
		let ret: CutForSearchResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
		};
		Ok(ret.words)
	} else {
		let err: ServiceErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
		};
		Err(err.to_service_error())
	}
}

pub async fn cut_for_index(text: String) -> Result<Vec<String>, ServiceError> {
	let url = format!("{}/i", TEXTSEG_ADDRESS);
	let client = reqwest::Client::new();
	let response = client.post(url.clone())
		.body(text)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(e) => { return Err(ServiceError::new_network_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
	};
	let status = response.status();
	if status.is_success() {
		let ret: CutForIndexResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
		};
		Ok(ret.words)
	} else {
		let err: ServiceErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { return Err(ServiceError::new_json_error(SERVICE_NAME, &url, Some(format!("{:?}", e)))); }
		};
		Err(err.to_service_error())
	}
}
