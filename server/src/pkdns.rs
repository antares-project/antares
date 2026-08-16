use crate::*;

pub type Timestamp = pkarr::Timestamp;
pub type BuildError = pkarr::errors::BuildError;
pub type PublicKeyError = pkarr::errors::PublicKeyError;
pub type PublishError = pkarr::errors::PublishError;
pub type SignedPacketBuildError = pkarr::errors::SignedPacketBuildError;
pub type ResourceRecord<'a> = pkarr::dns::ResourceRecord<'a>;
pub type Name<'a> = pkarr::dns::Name<'a>;
pub type TXT<'a> = pkarr::dns::rdata::TXT<'a>;
pub type Ipv4Addr = std::net::Ipv4Addr;
pub type Ipv6Addr = std::net::Ipv6Addr;

pub use pkarr::dns::rdata;

#[derive(Debug, Clone)]
pub struct Client {
	inner: pkarr::Client,
}

pub struct SignedPacket {
	inner: pkarr::SignedPacket,
}

#[derive(Debug, Clone, Default)]
pub struct SignedPacketBuilder {
	inner: pkarr::SignedPacketBuilder,
}

impl SignedPacket {
	pub fn builder() -> SignedPacketBuilder {
		SignedPacketBuilder::default()
	}
	pub fn public_key(&self) -> crypto::PublicKey {
		let bytes = self.inner.public_key().to_bytes();
		crypto::PublicKey::from_bytes(bytes).unwrap()
	}
	pub fn all_resource_records(&self) -> impl Iterator<Item = &ResourceRecord<'_>> {
		self.inner.all_resource_records()
	}
	pub fn resource_records(&self, name: &str) -> impl Iterator<Item = &ResourceRecord<'_>> {
		self.inner.resource_records(name)
	}
	pub fn timestamp(&self) -> Timestamp {
		self.inner.timestamp()
	}
}

impl SignedPacketBuilder {
	pub fn a(mut self, name: Name, ipv4: Ipv4Addr, ttl: u32) -> Self {
		self.inner = self.inner.a(name, ipv4, ttl);
		self
	}
	pub fn aaaa(mut self, name: Name, ipv6: Ipv6Addr, ttl: u32) -> Self {
		self.inner = self.inner.aaaa(name, ipv6, ttl);
		self
	}
	pub fn https(mut self, name: Name, https: rdata::SVCB, ttl: u32) -> Self {
		self.inner = self.inner.https(name, https, ttl);
		self
	}
	pub fn txt(mut self, name: Name, text: TXT, ttl: u32) -> Self {
		self.inner = self.inner.txt(name, text, ttl);
		self
	}
	pub fn sign(self, private_key: &crypto::PrivateKey) -> error::Result<SignedPacket> {
		let keypair = pkarr::Keypair::from_secret_key(private_key.as_bytes());
		let inner = self.inner.sign(&keypair)?;

		Ok(SignedPacket { inner })
	}
}

impl Client {
	pub fn new() -> error::Result<Self> {
		let inner = pkarr::ClientBuilder::default().build()?;

		Ok(Self { inner })
	}
	pub async fn resolve(&self, public_key: crypto::PublicKey) -> error::Result<Option<SignedPacket>> {
		let public_key = pkarr::PublicKey::try_from(&public_key.to_bytes() as &[u8])?;
		let inner = self.inner.resolve(&public_key).await;

		Ok(inner.map(|inner| SignedPacket { inner }))
	}
	pub async fn publish(&self, signed_packet: &SignedPacket, cas: Option<Timestamp>) -> error::Result<()> {
		Ok(self.inner.publish(&signed_packet.inner, cas).await?)
	}
}

pub async fn resolve_profile_servers(client: &Client, public_key: crypto::PublicKey) -> error::Result<Option<Vec<crypto::PublicKey>>> {
	let Some(signed_packet) = client.resolve(public_key).await? else {
		return Ok(None);
	};
	let mut servers = Vec::new();

	for record in signed_packet.resource_records("harmon") {
		let pkarr::dns::rdata::RData::TXT(txt) = &record.rdata else {
			log::warn!("Unexpected record type: {:?}", record);
			continue;
		};
		let Ok(server) = String::try_from(txt.clone()) else {
			log::warn!("Failed to parse TXT record as string: {:?}", txt);
			continue;
		};
		let Ok(public_key) = crypto::PublicKey::from_z32(&server) else {
			log::warn!("Failed to parse public key from server: {:?}", server);
			continue;
		};

		servers.push(public_key);
	}

	Ok(Some(servers))
}
