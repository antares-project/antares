use super::*;
use crate::*;

pub async fn connect(socket: wspc::Socket) -> error::Result<()> {
	log::info!("socket {} connected", socket.id());

	socket.join("global")?;
	socket.join(socket.id())?;

	socket.send("connectionReady", (socket.id(),))?;

	Ok(())
}

pub async fn disconnect(app: wspc::App, socket: wspc::Socket) -> error::Result<()> {
	log::info!("socket {} disconnected", socket.id());

	let state = app.get_state::<app::AppState>().unwrap();
	let Some(auth) = socket.get_state::<auth::AuthenticatedPayload>() else {
		return Ok(());
	};

	if let Some(channel) = socket.get_state::<channel::ChannelIdentifier>() {
		let Some(profile) = db::get_profile_by_public_key(&state.db_pool, auth.public_key).await? else {
			return Err(error::Error::ProfileDoesNotExist);
		};

		let room = app.room(channel);
		let member = channel::ChannelMember {
			profile: profile.into(),
			socket_id: socket.id(),
		};

		socket.leave(channel)?;
		room.emit("onChannelMemberLeft", (member,))?;
	};

	Ok(())
}
