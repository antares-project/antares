use super::*;
use crate::*;

use uuid::Uuid;

use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[repr(u8)]
pub enum ChannelType {
	Text = 0,
	Voice = 1,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
	#[serde(rename = "type")]
	pub r#type: ChannelType,
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

#[derive(Debug, serde::Serialize)]
pub struct ChannelMember {
	pub profile: profile::Profile,
	pub socket_id: Uuid,
}

#[derive(Debug, serde::Serialize)]
pub struct JoinChannelResponse {
	pub channel: Channel,
	pub members: Vec<ChannelMember>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ChannelIdentifier(pub Uuid);

pub async fn create_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<CreateChannelParams>) -> error::Result<Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel: Channel = db::create_channel(&state.db_pool, &params.name, db::ChannelType::Text).await?.into();

	app.room("global").emit("channelCreated", (&channel,))?;

	Ok(channel)
}

pub async fn delete_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<DeleteChannelParams>) -> error::Result<Channel> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_admin(&socket) {
		return Err(error::Error::Unauthorized);
	}

	let channel: Channel = db::delete_channel(&state.db_pool, params.channel_id).await?.into();

	app.room("global").emit("channelDeleted", (&channel,))?;

	Ok(channel)
}

pub async fn list_channels(app: wspc::App) -> error::Result<Vec<Channel>> {
	let state = app.get_state::<app::AppState>().unwrap();

	let channels = db::get_channels(&state.db_pool).await?.into_iter().map(Into::into).collect();

	Ok(channels)
}

pub async fn join_channel(app: wspc::App, socket: wspc::Socket, params: wspc::Params<JoinChannelParams>) -> error::Result<JoinChannelResponse> {
	let state = app.get_state::<app::AppState>().unwrap();

	if !auth::is_auth(&socket) {
		return Err(error::Error::Unauthorized);
	}

	if let Some(channel) = socket.get_state::<ChannelIdentifier>() {
		socket.leave(channel)?;
	};

	socket.join(params.channel_id)?;
	socket.set_state(params.channel_id);

	let channel: Channel = db::get_channel(&state.db_pool, *params.channel_id).await?.into();
	let sockets = app.room(params.channel_id).sockets();
	let mut members = Vec::new();

	for socket in sockets {
		let Some(auth) = socket.get_state::<auth::AuthenticatedPayload>() else { continue };
		let Some(profile) = db::get_profile_by_public_key(&state.db_pool, auth.public_key).await? else {
			continue;
		};

		members.push(ChannelMember {
			profile: profile.into(),
			socket_id: socket.id(),
		});
	}

	Ok(JoinChannelResponse { channel, members })
}

impl fmt::Display for ChannelIdentifier {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl ops::Deref for ChannelIdentifier {
	type Target = Uuid;
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<db::Channel> for Channel {
	#[inline(always)]
	fn from(value: db::Channel) -> Self {
		Self {
			id: value.id,
			name: value.name,
			r#type: value.r#type.into(),
		}
	}
}

impl From<db::ChannelType> for ChannelType {
	#[inline(always)]
	fn from(value: db::ChannelType) -> Self {
		match value {
			db::ChannelType::Text => Self::Text,
			db::ChannelType::Voice => Self::Voice,
		}
	}
}
