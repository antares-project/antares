use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct File {
	pub id: Uuid,
	pub name: String,
	pub hash: crypto::Hash32,
	pub size: i64,
	pub mime_type: String,
	pub counter: i64,
	pub created_at: time::OffsetDateTime,
}

pub async fn create_file(pool: impl sqlx::SqliteExecutor<'_>, name: &str, hash: crypto::Hash32, size: i64, mime_type: &str) -> error::Result<File> {
	let id = Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		File,
		r#"
			INSERT INTO files
				(id, name, hash, size, mime_type, created_at)
			VALUES
				(?, ?, ?, ?, ?, ?)
			RETURNING
				id as "id!: Uuid",
				name,
                hash as "hash!: crypto::Hash32",
                size,
                mime_type,
                counter,
				created_at as "created_at!: time::OffsetDateTime"
		;"#,
		id,
		name,
		hash,
		size,
		mime_type,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_file(pool: impl sqlx::SqliteExecutor<'_>, id: Uuid) -> error::Result<File> {
	Ok(sqlx::query_as!(
		File,
		r#"
			SELECT
				id as "id!: Uuid",
				name,
                hash as "hash!: crypto::Hash32",
				size,
				mime_type,
				counter,
				created_at as "created_at!: time::OffsetDateTime"
			FROM 
				files
			WHERE
				id = ?
		;"#,
		id
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_file_by_hash(pool: impl sqlx::SqliteExecutor<'_>, hash: crypto::Hash32) -> error::Result<File> {
	Ok(sqlx::query_as!(
		File,
		r#"
			SELECT
				id as "id!: Uuid",
				name,
                hash as "hash!: crypto::Hash32",
				size,
				mime_type,
				counter,
				created_at as "created_at!: time::OffsetDateTime"
			FROM 
				files
			WHERE
				hash = ?
		;"#,
		hash
	)
	.fetch_one(pool)
	.await?)
}

pub async fn increment_file_counter(pool: impl sqlx::SqliteExecutor<'_>, id: Uuid) -> error::Result<File> {
	Ok(sqlx::query_as!(
		File,
		r#"
            UPDATE
				files
            SET
                counter = counter + 1
            WHERE
                id = ?
            RETURNING
                id as "id!: Uuid",
				name,
                hash as "hash!: crypto::Hash32",
                size,
                mime_type,
                counter,
				created_at as "created_at!: time::OffsetDateTime"
        ;"#,
		id
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_files_from_message(pool: impl sqlx::SqliteExecutor<'_>, message_id: Uuid) -> error::Result<Vec<File>> {
	Ok(sqlx::query_as!(
		File,
		r#"
        SELECT
            file.id as "id!: Uuid",
			file.name,
            file.hash as "hash!: crypto::Hash32",
            file.size,
            file.mime_type,
            file.counter,
			file.created_at as "created_at!: time::OffsetDateTime"
        FROM
            files file
        INNER JOIN attachments
			attachment
        ON
			attachment.file_id = file.id
        WHERE
            attachment.message_id = ?
        ;"#,
		message_id
	)
	.fetch_all(pool)
	.await?)
}

pub async fn delete_unreferenced_files(pool: impl sqlx::SqliteExecutor<'_>, date: time::OffsetDateTime) -> error::Result<Vec<File>> {
	Ok(sqlx::query_as!(
		File,
		r#"
		DELETE FROM
			files
		WHERE
			created_at < ?1
				AND
			counter <= 0
		RETURNING
			id as "id!: Uuid",
			name,
            hash as "hash!: crypto::Hash32",
            size,
            mime_type,
            counter,
			created_at as "created_at!: time::OffsetDateTime"
		;"#,
		date
	)
	.fetch_all(pool)
	.await?)
}
