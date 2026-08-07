use crate::*;

pub async fn connect(socket: wspc::Socket) -> error::Result<()> {
	log::info!("New connection: {}", socket.id());
	socket.join("events")?;
	Ok(())
}
