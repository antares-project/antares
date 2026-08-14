pub mod app;
pub mod config;
pub mod crypto;
pub mod db;
pub mod env;
pub mod error;
pub mod jobs;
pub mod routes;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cli {
	GenerateKey,
	Run,
}

#[tokio::main]
async fn main() -> error::Result<()> {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	match Cli::parse() {
		Cli::GenerateKey => Ok(generate_key().await?),
		Cli::Run => Ok(run().await?),
	}
}

async fn run() -> error::Result<()> {
	let app = app::AppState::init().await?;

	jobs::init(&app).await;

	let listener = tokio::net::TcpListener::bind(&format!("{}:{}", app.env.server_addr, app.env.server_port)).await?;

	log::info!("Listening on {}:{}", app.env.server_addr, app.env.server_port);
	log::info!("Public Key: {}", app.env.public_key);

	let router = routes::get_routes(&app).await.with_state(app);

	Ok(axum::serve(listener, router).await?)
}

async fn generate_key() -> error::Result<()> {
	let private_key = crypto::PrivateKey::generate();
	let public_key = private_key.public_key();

	log::info!("Private Key: {}", private_key);
	log::info!("Public Key: {}", public_key);

	Ok(())
}
