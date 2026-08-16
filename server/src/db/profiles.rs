use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Profile {
	pub id: Uuid,
	pub public_key: crypto::PublicKey,
	pub name: String,
	pub updated_at: time::OffsetDateTime,
	pub created_at: time::OffsetDateTime,
}

pub async fn update_or_insert_profile(pool: &sqlx::sqlite::SqlitePool, public_key: crypto::PublicKey, name: &str) -> error::Result<Profile> {
	let id = Uuid::now_v7();
	let updated_at = time::OffsetDateTime::now_utc();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Profile,
		r#"
            INSERT INTO profiles
                (id, public_key, name, updated_at, created_at)
            VALUES
                (?, ?, ?, ?, ?)
			ON CONFLICT(public_key) DO UPDATE SET
				name = excluded.name,
				updated_at = excluded.updated_at
            RETURNING
                id as "id!: Uuid",
                public_key as "public_key!: crypto::PublicKey",
                name as "name!: String",
				updated_at as "updated_at!: time::OffsetDateTime",
				created_at as "created_at!: time::OffsetDateTime"
            ;
        "#,
		id,
		public_key,
		name,
		updated_at,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_profile_by_public_key(pool: &sqlx::sqlite::SqlitePool, public_key: crypto::PublicKey) -> error::Result<Option<Profile>> {
	Ok(sqlx::query_as!(
		Profile,
		r#"
			SELECT
				id as "id!: Uuid",
				public_key as "public_key!: crypto::PublicKey",
				name as "name!: String",
				updated_at as "updated_at!: time::OffsetDateTime",
				created_at as "created_at!: time::OffsetDateTime"
			FROM 
				profiles
			WHERE
				public_key = ?
			;
		"#,
		public_key
	)
	.fetch_optional(pool)
	.await?)
}
