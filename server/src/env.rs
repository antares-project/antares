use crate::*;

use std::env;

#[derive(Debug, Clone)]
pub struct Env {
	pub server_addr: String,
	pub server_port: u16,
	pub config_path: String,
	pub database_url: String,
	pub jwt_secret: String,
	pub jwt_expiration_seconds: u64,
	pub jwt_challenge_secret: String,
	pub jwt_challenge_expiration_seconds: u64,
	pub private_key: crypto::PrivateKey,
	pub public_key: crypto::PublicKey,
}

impl Env {
	pub fn init() -> error::Result<Self> {
		let _ = dotenvy::dotenv();

		let server_addr = env::var("SERVER_ADDR")?;
		let server_port = env::var("SERVER_PORT")?.parse()?;

		let config_path = env::var("CONFIG_PATH")?;
		let database_url = env::var("DATABASE_URL")?;

		let jwt_secret = env::var("JWT_SECRET")?;
		let jwt_expiration_seconds = env::var("JWT_EXPIRATION_SECONDS")?.parse()?;
		let jwt_challenge_secret = env::var("JWT_CHALLENGE_SECRET")?;
		let jwt_challenge_expiration_seconds = env::var("JWT_CHALLENGE_EXPIRATION_SECONDS")?.parse()?;

		let private_key = crypto::PrivateKey::from_bytes(hex::FromHex::from_hex(env::var("PRIVATE_KEY")?)?);
		let public_key = private_key.public_key();

		Ok(Self {
			server_addr,
			server_port,
			config_path,
			database_url,
			jwt_secret,
			jwt_expiration_seconds,
			jwt_challenge_secret,
			jwt_challenge_expiration_seconds,
			private_key,
			public_key,
		})
	}
}
