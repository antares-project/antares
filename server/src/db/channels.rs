use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
	pub created_at: time::OffsetDateTime,
}

pub async fn create_channel(pool: &sqlx::sqlite::SqlitePool, name: &str) -> error::Result<Channel> {
	let id = Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Channel,
		r#"
			INSERT INTO channels
				(id, name, created_at)
			VALUES
				(?, ?, ?)
			RETURNING
				id as "id!: Uuid",
				name,
				created_at as "created_at!: time::OffsetDateTime"
		;"#,
		id,
		name,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn delete_channel(pool: &sqlx::sqlite::SqlitePool, id: Uuid) -> error::Result<Channel> {
	Ok(sqlx::query_as!(
		Channel,
		r#"
			DELETE FROM
				channels
			WHERE
				id = ?
			RETURNING
				id as "id!: Uuid",
				name,
				created_at as "created_at!: time::OffsetDateTime"
		;"#,
		id
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_channels(pool: &sqlx::sqlite::SqlitePool) -> error::Result<Vec<Channel>> {
	Ok(sqlx::query_as!(
		Channel,
		r#"
			SELECT
				id as "id!: Uuid",
				name,
				created_at as "created_at!: time::OffsetDateTime"
			FROM 
				channels
			ORDER BY
				id ASC
		;"#,
	)
	.fetch_all(pool)
	.await?)
}

pub async fn get_channel(pool: &sqlx::sqlite::SqlitePool, id: Uuid) -> error::Result<Channel> {
	Ok(sqlx::query_as!(
		Channel,
		r#"
			SELECT
				id as "id!: Uuid",
				name,
				created_at as "created_at!: time::OffsetDateTime"
			FROM 
				channels
			WHERE
				id = ?
		;"#,
		id,
	)
	.fetch_one(pool)
	.await?)
}
