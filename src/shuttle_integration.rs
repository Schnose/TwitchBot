use {crate::state::State, shuttle_runtime::async_trait, std::net::SocketAddr};

pub type ShuttleResult = Result<State, shuttle_service::Error>;

#[async_trait]
impl shuttle_service::Service for State {
	async fn bind(self, _: SocketAddr) -> Result<(), shuttle_service::Error> {
		self.run()
			.await
			.expect("Failed to run Twitch Bot.");

		Ok(())
	}
}
