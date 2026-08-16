mod auth;
mod channel;
mod life_cycle;
mod message;
mod profile;

use crate::*;

use axum::routing::*;

pub fn router(state: &app::AppState) -> MethodRouter<app::AppState> {
	let (route, app) = wspc::App::build_route();

	app.set_state(state.clone());

	app.on("connect", life_cycle::connect);
	app.on("disconnect", life_cycle::disconnect);

	app.on("auth", auth::auth);
	app.on("requestChallenge", auth::request_challenge);
	app.on("confirmChallenge", auth::confirm_challenge);

	app.on("sendMessage", message::send_message);
	app.on("loadMessages", message::load_messages);

	app.on("joinChannel", channel::join_channel);
	app.on("createChannel", channel::create_channel);
	app.on("deleteChannel", channel::delete_channel);
	app.on("listChannels", channel::list_channels);

	app.on("getProfile", profile::get_profile);
	app.on("updateProfile", profile::update_profile);

	route
}
