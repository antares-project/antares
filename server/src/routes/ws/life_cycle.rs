use crate::*;

pub async fn connect(socket: wspc::Socket) -> error::Result<()> {
	log::info!("socket {} connected", socket.id());

	socket.join("global")?;
	socket.join(socket.id())?;

	Ok(())
}

pub async fn disconnect(socket: wspc::Socket) -> error::Result<()> {
	log::info!("socket {} disconnected", socket.id());

	Ok(())
}
