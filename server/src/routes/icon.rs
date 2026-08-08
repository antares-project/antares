use crate::*;

use std::fs;

use axum::extract::*;
use axum::routing::*;

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", get(get_icon_route))
}

async fn get_icon_route(app: State<app::AppState>) -> error::Result<axum::response::Response> {
	let icon_path = &app.config.icon_path;
	if !icon_path.exists() {
		return Err(error::Error::IconNotFound);
	}
	let mime_type = match mime_guess::from_path(icon_path).first_raw() {
		Some(mime) => mime,
		None => return Err(error::Error::IconNotFound),
	};
	let data = fs::read(&icon_path)?;
	let response = axum::response::Response::builder().header("Content-Type", mime_type).body(axum::body::Body::from(data)).unwrap();

	Ok(response)
}
