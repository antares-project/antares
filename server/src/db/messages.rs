use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
	pub id: Uuid,
	pub channel_id: Uuid,
	pub profile_id: Uuid,
	pub content: String,
	pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct MessageWithFiles {
	pub id: Uuid,
	pub channel_id: Uuid,
	pub profile_id: Uuid,
	pub content: String,
	pub files: Vec<db::File>,
	pub created_at: time::OffsetDateTime,
}

pub async fn create_message(pool: impl sqlx::SqliteExecutor<'_>, channel_id: Uuid, profile_id: Uuid, content: &str) -> error::Result<Message> {
	let id = Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Message,
		r#"
            INSERT INTO messages
                (id, channel_id, profile_id, content, created_at)
            VALUES
                (?, ?, ?, ?, ?)
            RETURNING
                id as "id!: Uuid",
                channel_id as "channel_id!: Uuid",
                profile_id as "profile_id!: Uuid",
                content as "content!",
				created_at as "created_at!: time::OffsetDateTime"
            ;
        "#,
		id,
		channel_id,
		profile_id,
		content,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_messages(pool: impl sqlx::SqliteExecutor<'_>, channel_id: Uuid, before_id: Option<Uuid>, limit: i64) -> error::Result<Vec<Message>> {
	Ok(sqlx::query_as!(
		Message,
		r#"
            SELECT
                id as "id!: Uuid",
                channel_id as "channel_id!: Uuid",
                content as "content!",
                profile_id as "profile_id!: Uuid",
				created_at as "created_at!: time::OffsetDateTime"
            FROM (
                SELECT
                    id, channel_id, profile_id, content, created_at
                FROM
                    messages
                WHERE
                    channel_id = ?1 AND (?2 IS NULL OR id < ?2)
                ORDER BY
                    id DESC
                LIMIT
                    ?3
            )
            ORDER BY id ASC
        "#,
		channel_id,
		before_id,
		limit
	)
	.fetch_all(pool)
	.await?)
}

pub async fn get_message(pool: impl sqlx::SqliteExecutor<'_>, id: Uuid) -> error::Result<Message> {
	Ok(sqlx::query_as!(
		Message,
		r#"
			SELECT
                id as "id!: Uuid",
                channel_id as "channel_id!: Uuid",
                content as "content!",
                profile_id as "profile_id!: Uuid",
				created_at as "created_at!: time::OffsetDateTime"
			FROM
				messages
			WHERE
				id = ?1
		;"#,
		id
	)
	.fetch_one(pool)
	.await?)
}
