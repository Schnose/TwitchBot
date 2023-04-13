use {
	crate::{axum, state::State},
	shuttle_runtime::async_trait,
	std::net::SocketAddr,
};

pub type ShuttleResult = Result<State, shuttle_service::Error>;

#[async_trait]
impl shuttle_service::Service for State {
	async fn bind(self, addr: SocketAddr) -> Result<(), shuttle_service::Error> {
		tokio::spawn(axum::run(
			addr,
			self.config.streamers_table.clone(),
			self.database_connection.clone(),
		));

		self.run()
			.await
			.expect("Failed to run Twitch Bot.");

		Ok(())
	}
}
