use super::*;
use crate::*;

use uuid::Uuid;

const MESSAGE_PAGE_SIZE: i64 = 32;

#[derive(Debug, serde::Deserialize)]
pub struct SendMessageParams {
	pub content: String,
	pub attachments: Vec<Uuid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoadMessagesParams {
	pub before_id: Option<Uuid>,
}

#[derive(Debug, serde::Serialize)]
pub struct Message {
	pub id: Uuid,
	pub channel_id: Uuid,
	pub profile: profile::Profile,
	pub content: String,
	pub attachments: Vec<MessageAttachment>,
}

#[derive(Debug, serde::Serialize)]
pub struct MessageAttachment {
	pub id: Uuid,
	pub name: String,
	pub mime_type: String,
	pub size: i64,
	pub hash: crypto::Hash32,
}

pub async fn send_message(app: wspc::App, socket: wspc::Socket, params: wspc::Params<SendMessageParams>) -> error::Result<()> {
	let state = app.get_state::<app::AppState>().unwrap();
	let Some(auth) = socket.get_state::<auth::AuthenticatedPayload>() else {
		return Err(error::Error::Unauthorized);
	};

	let Some(channel) = socket.get_state::<channel::ChannelIdentifier>() else {
		return Err(error::Error::NotInChannel);
	};

	let mut tx = state.db_pool.begin().await?;

	let Some(profile) = db::get_profile_by_public_key(&mut *tx, auth.public_key).await? else {
		return Err(error::Error::ProfileDoesNotExist);
	};
	let message = db::create_message(&mut *tx, channel.0, profile.id, &params.content).await?;

	for file_id in &params.attachments {
		db::create_attachment(&mut *tx, message.id, *file_id).await?;
		db::increment_file_counter(&mut *tx, *file_id).await?;
	}

	tx.commit().await?;

	let message = db::get_message(&state.db_pool, message.id).await?;
	let profile = db::get_profile(&state.db_pool, message.profile_id).await?;
	let files = db::get_files_from_message(&state.db_pool, message.id).await?;

	let message = Message {
		id: message.id,
		channel_id: message.channel_id,
		profile: profile.into(),
		content: message.content,
		attachments: files.into_iter().map(Into::into).collect(),
	};

	app.room(channel).emit("messageReceived", (message,))?;

	Ok(())
}

pub async fn load_messages(app: wspc::App, socket: wspc::Socket, params: wspc::Params<LoadMessagesParams>) -> error::Result<Vec<Message>> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let Some(channel) = socket.get_state::<channel::ChannelIdentifier>() else {
		return Err(error::Error::NotInChannel);
	};

	let mut messages = Vec::new();

	for message in db::get_messages(&state.db_pool, channel.0, params.before_id, MESSAGE_PAGE_SIZE).await? {
		let profile = db::get_profile(&state.db_pool, message.profile_id).await?;
		let files = db::get_files_from_message(&state.db_pool, message.id).await?;

		messages.push(Message {
			id: message.id,
			channel_id: message.channel_id,
			profile: profile.into(),
			content: message.content,
			attachments: files.into_iter().map(Into::into).collect(),
		});
	}

	Ok(messages)
}

impl From<db::File> for MessageAttachment {
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
