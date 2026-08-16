use crate::*;

use std::fs;

#[macro_export]
macro_rules! create_interval {
	($millis:expr, $closure:expr) => {
		let duration = std::time::Duration::from_millis($millis);
		let mut interval = tokio::time::interval(duration);

		loop {
			interval.tick().await;
			$closure
		}
	};
}

pub async fn init(app: &app::AppState) {
	let app_state = app.clone();
	tokio::spawn(db_migration_job(app_state));
	let app_state = app.clone();
	tokio::spawn(clear_old_files(app_state));
	let app_state = app.clone();
	tokio::spawn(dns_publisher_job(app_state));
}

async fn db_migration_job(app: app::AppState) -> error::Result<()> {
	Ok(db::MIGRATOR.run(&app.db_pool).await?)
}

async fn clear_old_files(app: app::AppState) -> error::Result<()> {
	create_interval!(300_000, {
		let date = time::OffsetDateTime::now_utc() - time::Duration::minutes(5);
		let files = db::delete_unreferenced_files(&app.db_pool, date).await?;

		for file in files {
			let name = hex::encode(*file.hash);
			let path = app.config.file_path.join(name);

			log::info!("deleting file {:?}", file.hash);
			fs::remove_file(path)?;
		}
	});
}

async fn dns_publisher_job(app: app::AppState) -> error::Result<()> {
	let ttl = 300;
	let dns = pkdns::DnsServer {
		a: app.config.public_ipv4_address,
		aaaa: app.config.public_ipv6_address,
		https: app.config.public_https_address,
	};

	let packet = dns.to_signed_packet_builder(ttl)?.sign(&app.env.private_key)?;

	create_interval!(3_600_000, {
		app.pkdns.publish(&packet, None).await?;
	});
}
