use super::*;
use crate::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Profile {
	pub public_key: crypto::PublicKey,
	pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateProfileParams {
	pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GetProfileParams {
	pub public_key: Option<crypto::PublicKey>,
}

pub async fn update_profile(params: wspc::Params<UpdateProfileParams>, app: wspc::App, socket: wspc::Socket) -> error::Result<Profile> {
	let state = app.get_state::<app::AppState>().unwrap();

	let Some(authenticated) = socket.get_state::<auth::AuthenticatedPayload>() else {
		return Err(error::Error::Unauthorized);
	};

	let profile = db::update_or_insert_profile(&state.db_pool, authenticated.public_key, &params.name).await?;

	Ok(profile.into())
}

pub async fn get_profile(params: wspc::Params<GetProfileParams>, app: wspc::App, socket: wspc::Socket) -> error::Result<Option<Profile>> {
	let state = app.get_state::<app::AppState>().unwrap();

	let public_key = match params.public_key {
		Some(public_key) => public_key,
		None => match socket.get_state::<auth::AuthenticatedPayload>() {
			Some(authenticated) => authenticated.public_key,
			None => return Err(error::Error::Unauthorized),
		},
	};

	let profile = db::get_profile_by_public_key(&state.db_pool, public_key).await?;

	Ok(profile.map(|profile| profile.into()))
}

impl From<db::Profile> for Profile {
	#[inline(always)]
	fn from(profile: db::Profile) -> Self {
		Self {
			public_key: profile.public_key,
			name: profile.name,
		}
	}
}
