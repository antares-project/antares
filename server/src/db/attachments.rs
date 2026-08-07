use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Attachment {
	pub id: Uuid,
	pub message_id: Uuid,
	pub file_id: Uuid,
	pub created_at: time::OffsetDateTime,
}

pub async fn create_attachment(pool: impl sqlx::SqliteExecutor<'_>, message_id: Uuid, file_id: Uuid) -> error::Result<Attachment> {
	let id = Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Attachment,
		r#"
            INSERT INTO attachments
                (id, message_id, file_id, created_at)
            VALUES
                (?, ?, ?, ?)
            RETURNING
                id as "id!: Uuid",
                message_id as "message_id!: Uuid",
                file_id as "file_id!: Uuid",
                created_at as "created_at!: time::OffsetDateTime"
            ;
        "#,
		id,
		message_id,
		file_id,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn delete_attachment(pool: impl sqlx::SqliteExecutor<'_>, id: Uuid) -> error::Result<Attachment> {
	Ok(sqlx::query_as!(
		Attachment,
		r#"
            DELETE FROM
                attachments
            WHERE
                id = ?
            RETURNING
                id as "id!: Uuid",
                message_id as "message_id!: Uuid",
                file_id as "file_id!: Uuid",
                created_at as "created_at!: time::OffsetDateTime"
            ;
        "#,
		id
	)
	.fetch_one(pool)
	.await?)
}
