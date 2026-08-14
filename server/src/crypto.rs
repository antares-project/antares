use crate::*;

use std::fmt;
use std::ops;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(ed25519_dalek::VerifyingKey);

#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey(ed25519_dalek::SigningKey);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(ed25519_dalek::Signature);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash32([u8; 32]);

pub fn rand32() -> [u8; 32] {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	secret
}

pub fn sha256(data: &[u8]) -> Hash32 {
	Hash32(sha2::Sha256::digest(data).as_slice().try_into().unwrap())
}

pub fn encode_jwt<T: serde::ser::Serialize>(secret: &[u8], claims: &T) -> error::Result<String> {
	let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret);
	Ok(jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &encoding_key)?)
}

pub fn decode_jwt<T: serde::de::DeserializeOwned>(secret: &[u8], token: &str) -> error::Result<T> {
	let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret);
	Ok(jsonwebtoken::decode::<T>(token, &decoding_key, &jsonwebtoken::Validation::default())?.claims)
}

impl PublicKey {
	#[inline(always)]
	pub fn verify(&self, message: &[u8], signature: Signature) -> bool {
		self.0.verify_strict(message, &signature.0).is_ok()
	}
	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8; 32] {
		self.0.as_bytes()
	}
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 32] {
		self.0.to_bytes()
	}
	pub fn from_bytes(bytes: [u8; 32]) -> error::Result<Self> {
		Ok(Self(ed25519_dalek::VerifyingKey::from_bytes(&bytes)?))
	}
	pub fn to_z32(&self) -> String {
		base32::encode(base32::Alphabet::Z, &self.0.to_bytes())
	}
	pub fn from_z32(value: &str) -> error::Result<Self> {
		let Some(bytes) = base32::decode(base32::Alphabet::Z, value) else {
			return Err(error::Error::FailedDecodeBase32);
		};
		let Ok(array) = bytes.try_into() else {
			return Err(error::Error::InvalidLength);
		};
		Ok(Self::from_bytes(array)?)
	}
}

impl PrivateKey {
	#[inline(always)]
	pub fn generate() -> Self {
		Self::from_bytes(rand32())
	}
	#[inline(always)]
	pub fn public_key(&self) -> PublicKey {
		PublicKey(self.0.verifying_key())
	}
	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8; 32] {
		self.0.as_bytes()
	}
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 32] {
		self.0.to_bytes()
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 32]) -> Self {
		Self(ed25519_dalek::SigningKey::from_bytes(&bytes))
	}
	pub fn to_z32(&self) -> String {
		base32::encode(base32::Alphabet::Z, &self.0.to_bytes())
	}
	pub fn from_z32(value: &str) -> error::Result<Self> {
		let Some(bytes) = base32::decode(base32::Alphabet::Z, value) else {
			return Err(error::Error::FailedDecodeBase32);
		};
		let Ok(array) = bytes.try_into() else {
			return Err(error::Error::InvalidLength);
		};
		Ok(Self(ed25519_dalek::SigningKey::from_bytes(&array)))
	}
}

impl Signature {
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 64] {
		self.0.to_bytes()
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 64]) -> Self {
		Self(ed25519_dalek::Signature::from_bytes(&bytes))
	}
	pub fn to_z32(&self) -> String {
		base32::encode(base32::Alphabet::Z, &self.0.to_bytes())
	}
	pub fn from_z32(value: &str) -> error::Result<Self> {
		let Some(bytes) = base32::decode(base32::Alphabet::Z, value) else {
			return Err(error::Error::FailedDecodeBase32);
		};
		let Ok(array) = bytes.try_into() else {
			return Err(error::Error::InvalidLength);
		};
		Ok(Self::from_bytes(array))
	}
}

impl Hash32 {
	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 32] {
		self.0
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
	pub fn to_hex(&self) -> String {
		hex::encode(self.to_bytes())
	}
	pub fn from_hex(value: &str) -> error::Result<Self> {
		let bytes = hex::decode(value)?;
		Ok(Self::from_bytes(bytes.as_slice().try_into().map_err(|_| error::Error::InvalidLength)?))
	}
}

impl serde::Serialize for PublicKey {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_z32().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for PublicKey {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let value = String::deserialize(deserializer)?;

		Ok(Self::from_z32(&value).map_err(|_| serde::de::Error::custom("invalid public key"))?)
	}
}

impl serde::Serialize for Signature {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_z32().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Signature {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let value = String::deserialize(deserializer)?;

		Ok(Self::from_z32(&value).map_err(|_| serde::de::Error::custom("invalid signature"))?)
	}
}

impl std::fmt::Debug for PublicKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl std::fmt::Debug for PrivateKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl std::fmt::Debug for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl std::fmt::Display for PublicKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl std::fmt::Display for PrivateKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl std::fmt::Display for Signature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.to_z32())
	}
}

impl sqlx::Type<sqlx::Sqlite> for Hash32 {
	#[inline(always)]
	fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
		<Vec<u8> as sqlx::Type<sqlx::Sqlite>>::type_info()
	}
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Hash32 {
	fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		let value = self.0.to_vec();

		<Vec<u8> as sqlx::Encode<sqlx::Sqlite>>::encode(value, args)
	}
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Hash32 {
	fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
		let bytes = <Vec<u8> as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
		let value = bytes.as_slice().try_into().map_err(|_| "invalid hash length (expected 32 bytes)")?;

		Ok(Self::from_bytes(value))
	}
}

impl sqlx::Type<sqlx::Sqlite> for PublicKey {
	#[inline(always)]
	fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
		<Vec<u8> as sqlx::Type<sqlx::Sqlite>>::type_info()
	}
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for PublicKey {
	fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		let value = self.0.as_bytes().to_vec();

		<Vec<u8> as sqlx::Encode<sqlx::Sqlite>>::encode(value, args)
	}
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for PublicKey {
	fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
		let bytes = <Vec<u8> as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
		let value = bytes.as_slice().try_into().map_err(|_| "invalid hash length (expected 32 bytes)")?;

		Ok(Self::from_bytes(value).map_err(|_| "Invalid public key bytes")?)
	}
}

impl serde::Serialize for Hash32 {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_hex().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Hash32 {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		Ok(Self::from_hex(&hex).map_err(|_| serde::de::Error::custom("invalid hash"))?)
	}
}

impl fmt::Debug for Hash32 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.to_hex())
	}
}

impl ops::Deref for Hash32 {
	type Target = [u8; 32];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for Hash32 {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
