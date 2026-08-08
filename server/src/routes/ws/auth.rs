use crate::*;

use std::time;

#[derive(Debug, Clone)]
pub struct AuthenticatedData {
	#[allow(dead_code)]
	pub public_key: crypto::PublicKey,
	pub is_admin: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthParams {
	pub token: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthChallengeParams {
	pub public_key: crypto::PublicKey,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfirmAuthChallengeParams {
	pub token: String,
	pub signature: crypto::Signature,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedPayload {
	pub public_key: crypto::PublicKey,
	pub is_admin: bool,
	pub exp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChallengePayload {
	pub public_key: crypto::PublicKey,
	pub nonce: [u8; 32],
	pub exp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseConfirmAuthChallenge {
	pub token: String,
	pub payload: AuthenticatedPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseAuthChallenge {
	pub token: String,
}

#[inline]
pub fn is_admin(socket: &wspc::Socket) -> bool {
	if let Some(AuthenticatedData { is_admin, .. }) = socket.get_state::<AuthenticatedData>() {
		is_admin
	} else {
		false
	}
}

#[inline]
pub fn is_auth(socket: &wspc::Socket) -> bool {
	socket.get_state::<AuthenticatedData>().is_some()
}

pub async fn auth(app: wspc::App, socket: wspc::Socket, params: wspc::Params<AuthParams>) -> error::Result<AuthenticatedPayload> {
	let state = app.get_state::<app::AppState>().unwrap();

	let jwt_secret = state.env.jwt_secret.as_bytes();
	let data = crypto::decode_jwt::<AuthenticatedPayload>(jwt_secret, &params.token)?;

	let public_key = data.public_key;
	let is_admin = data.is_admin;

	socket.set_state(AuthenticatedData { public_key, is_admin });

	Ok(data)
}

pub async fn request_challenge(app: wspc::App, params: wspc::Params<AuthChallengeParams>) -> error::Result<ResponseAuthChallenge> {
	let state = app.get_state::<app::AppState>().unwrap();

	let nonce = crypto::rand32();
	let public_key = params.public_key;

	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + state.env.jwt_challenge_expiration_seconds;

	let challenge_payload = ChallengePayload { public_key, nonce, exp };
	let token = crypto::encode_jwt(state.env.jwt_challenge_secret.as_bytes(), &challenge_payload)?;

	Ok(ResponseAuthChallenge { token })
}

pub async fn confirm_challenge(app: wspc::App, params: wspc::Params<ConfirmAuthChallengeParams>) -> error::Result<ResponseConfirmAuthChallenge> {
	let state = app.get_state::<app::AppState>().unwrap();

	let token = &params.token;
	let signature = params.signature;
	let token_hash = crypto::sha256(token.as_bytes());
	let challenge_payload = crypto::decode_jwt::<ChallengePayload>(state.env.jwt_challenge_secret.as_bytes(), token)?;
	let public_key = challenge_payload.public_key;

	let valid = public_key.verify(&*token_hash, signature);

	if !valid {
		return Err(error::Error::Unauthorized);
	}

	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + state.env.jwt_expiration_seconds;

	let is_admin = state.config.admin_public_keys.contains(public_key.as_bytes());
	let payload = AuthenticatedPayload { public_key, is_admin, exp };
	let token = crypto::encode_jwt(state.env.jwt_secret.as_bytes(), &payload)?;

	Ok(ResponseConfirmAuthChallenge { token, payload })
}
