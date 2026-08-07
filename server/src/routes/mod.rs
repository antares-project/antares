mod files;
mod icon;
mod info;
mod root;
mod ws;

use axum::extract::*;

use crate::*;

pub async fn get_routes(app: &app::AppState) -> axum::Router<app::AppState> {
	let cors = tower_http::cors::CorsLayer::new()
		.allow_headers(tower_http::cors::Any)
		.allow_origin(tower_http::cors::Any)
		.allow_methods(tower_http::cors::Any);

	let body_limit = DefaultBodyLimit::disable();

	axum::Router::new()
		.merge(root::router())
		.nest("/info", info::router())
		.nest("/icon", icon::router())
		.nest("/files", files::router())
		.route("/ws", ws::router(app))
		.layer(cors)
		.layer(body_limit)
}
