use crate::*;

use uuid::Uuid;

#[repr(u8)]
#[derive(Debug, Clone, Copy, sqlx::Type)]
pub enum ChannelType {
	Text = 0,
	Voice = 1,
}

#[derive(Debug, Clone)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
	pub r#type: ChannelType,
	pub created_at: time::OffsetDateTime,
}

pub async fn create_channel(pool: &sqlx::sqlite::SqlitePool, name: &str, r#type: ChannelType) -> error::Result<Channel> {
	let id = Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Channel,
		r#"
			INSERT INTO channels
				(id, name, type, created_at)
			VALUES
				(?, ?, ?, ?)
			RETURNING
				id as "id!: Uuid",
				name,
				type as "type: ChannelType",
				created_at as "created_at!: time::OffsetDateTime"
		;"#,
		id,
		name,
		r#type,
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
				type as "type: ChannelType",
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
				type as "type: ChannelType",
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
				type as "type: ChannelType",
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
