use crate::*;

#[derive(Clone)]
pub struct AppState {
	pub env: env::Env,
	pub config: config::Config,
	pub db_pool: sqlx::sqlite::SqlitePool,
	pub pkdns: pkarr::Client,
}

impl AppState {
	pub async fn init() -> error::Result<Self> {
		let env = env::Env::init()?;
		let config = config::Config::init(&env)?;
		let db_pool = db::connect(&env.database_url).await?;
		let pkdns = pkarr::ClientBuilder::default().build()?;

		Ok(Self { env, config, db_pool, pkdns })
	}
}
