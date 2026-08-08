use crate::*;

use axum::extract::*;
use axum::routing::*;

use tokio::fs;
use tokio_util::io;

#[derive(Debug, serde::Serialize)]
pub struct File {
	pub id: uuid::Uuid,
	pub name: String,
	pub mime_type: String,
	pub size: i64,
	pub hash: crypto::Hash32,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", post(post_file)).route("/{id}", get(get_file))
}

pub async fn post_file(app: State<app::AppState>, mut multipart: Multipart) -> error::Result<Json<Vec<File>>> {
	let mut files = Vec::new();

	while let Ok(Some(field)) = multipart.next_field().await {
		let name = field.file_name();
		let mime = match field.content_type() {
			Some(mime) => Some(mime),
			None => match name {
				Some(name) => mime_guess::from_path(name).first_raw(),
				None => None,
			},
		};
		let name = name.unwrap_or("unknown").to_string();
		let mime = mime.unwrap_or("application/octet-stream").to_string();

		let Ok(data) = field.bytes().await else {
			continue;
		};

		let hash = crypto::sha256(&data);
		let size = data.len() as i64;
		let path = app.config.file_path.join(hex::encode(*hash));

		if let Ok(file) = db::get_file_by_hash(&app.db_pool, hash).await {
			log::info!(r#"file "{:?}" already exists, skipping the write"#, file.hash);
			files.push(file.into());
			continue;
		}

		let mut tx = app.db_pool.begin().await?;

		let file = db::create_file(&mut *tx, &name, hash, size, &mime).await?;

		fs::create_dir_all(&app.config.file_path).await?;
		fs::write(path, data).await?;

		tx.commit().await?;

		files.push(file.into());
	}

	Ok(Json(files))
}

pub async fn get_file(app: State<app::AppState>, Path(id): Path<uuid::Uuid>) -> error::Result<axum::response::Response> {
	let file = db::get_file(&app.db_pool, id).await?;

	let mime = file.mime_type;
	let size = file.size;
	let name = file.name;
	let path = app.config.file_path.join(hex::encode(*file.hash));
	let disposition = format!("attachment; filename*=UTF-8''{}", urlencoding::encode(&name));

	let file = fs::File::open(path).await?;
	let stream = io::ReaderStream::new(file);

	let response = axum::response::Response::builder()
		.header(axum::http::header::CONTENT_TYPE, mime)
		.header(axum::http::header::CONTENT_LENGTH, size)
		.header(axum::http::header::CONTENT_DISPOSITION, disposition)
		.body(axum::body::Body::from_stream(stream))
		.unwrap();

	Ok(response)
}

impl From<db::File> for File {
	#[inline(always)]
	fn from(value: db::File) -> Self {
		Self {
			id: value.id,
			name: value.name,
			mime_type: value.mime_type,
			size: value.size,
			hash: value.hash,
		}
	}
}
