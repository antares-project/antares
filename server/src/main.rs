pub mod app;
pub mod config;
pub mod crypto;
pub mod db;
pub mod env;
pub mod error;
pub mod jobs;
pub mod routes;

#[tokio::main]
async fn main() -> error::Result<()> {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let app = app::AppState::init().await?;

	jobs::init(&app).await;

	let listener = tokio::net::TcpListener::bind(&format!("{}:{}", app.env.server_addr, app.env.server_port)).await?;

	log::info!("Listening on {}:{}", app.env.server_addr, app.env.server_port);
	log::info!("PublicKey {:?}", app.env.public_key);

	let router = routes::get_routes(&app).await.with_state(app);

	Ok(axum::serve(listener, router).await?)
}
