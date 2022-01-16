
use actix_web::{App, HttpServer, web};

mod clients;
mod schema;
mod route;
mod autotag;

pub static SERVICE_NAME: &'static str = "pvrustsvc-autotag";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	// TODO: add log

	HttpServer::new(move || {
		App::new()
			.route("/v1/infer-tags", web::post().to(route::v1_infer_tag))
	})
	.bind(("0.0.0.0", 80))?
	.run()
	.await
}
