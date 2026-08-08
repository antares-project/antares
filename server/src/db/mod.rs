use crate::*;

mod attachments;
mod channels;
mod files;
mod messages;

pub use attachments::*;
pub use channels::*;
pub use files::*;
pub use messages::*;

use sqlx::migrate::MigrateDatabase;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn connect(database_url: &str) -> error::Result<sqlx::sqlite::SqlitePool> {
	if !sqlx::Sqlite::database_exists(database_url).await.unwrap_or(false) {
		log::info!("Database does not exist, creating...");
		sqlx::Sqlite::create_database(database_url).await?;
	}

	return Ok(sqlx::sqlite::SqlitePoolOptions::new().connect(database_url).await?);
}
