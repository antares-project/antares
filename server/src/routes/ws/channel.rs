use super::*;
use crate::*;

use uuid::Uuid;

use std::fmt;
use std::ops;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateChannelParams {
	pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteChannelParams {
	pub channel_id: uuid::Uuid,
}

#[derive(Debug, serde::Deserialize)]
pub struct JoinChannelParams {
	pub channel_id: ChannelIdentifier,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ChannelIdentifier(pub Uuid);

pub async fn create_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<CreateChannelParams>) -> error::Result<Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel: Channel = db::create_channel(&state.db_pool, &params.name).await?.into();

	app.room("events").emit("channelCreated", (&channel,))?;

	Ok(channel)
}

pub async fn delete_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<DeleteChannelParams>) -> error::Result<Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel: Channel = db::delete_channel(&state.db_pool, params.channel_id).await?.into();

	app.room("events").emit("channelDeleted", (&channel,))?;

	Ok(channel)
}

pub async fn list_channels(app: wspc::App) -> error::Result<Vec<Channel>> {
	let state = app.get_state::<app::AppState>().unwrap();

	let channels = db::get_channels(&state.db_pool).await?.into_iter().map(Into::into).collect();

	Ok(channels)
}

pub async fn join_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<JoinChannelParams>) -> error::Result<Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel = db::get_channel(&state.db_pool, *params.channel_id).await?.into();

	if let Some(channel) = socket.get_state::<ChannelIdentifier>() {
		socket.leave(channel)?;
	};

	socket.join(params.channel_id)?;
	socket.set_state(params.channel_id);

	Ok(channel)
}

impl fmt::Display for ChannelIdentifier {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ops::Deref for ChannelIdentifier {
	type Target = Uuid;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<db::Channel> for Channel {
	#[inline(always)]
	fn from(value: db::Channel) -> Self {
		Self { id: value.id, name: value.name }
	}
}
