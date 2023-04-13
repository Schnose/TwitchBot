use {
	crate::{config::Config, error::Result, tokenstore::TokenStore},
	schnosebot::global_map::GlobalMap,
	sqlx::{postgres::PgPoolOptions, Pool, Postgres},
	tokio::sync::mpsc::UnboundedReceiver,
	tracing::{error, info},
	twitch_irc::{
		login::RefreshingLoginCredentials,
		message::ServerMessage,
		transport::tcp::{TCPTransport, TLS},
		ClientConfig, TwitchIRCClient,
	},
};

type TwitchClient = TwitchIRCClient<TCPTransport<TLS>, RefreshingLoginCredentials<TokenStore>>;

#[derive(Debug)]
pub struct State {
	pub config: Config,
	pub twitch_client: TwitchClient,
	pub message_stream: UnboundedReceiver<ServerMessage>,
	pub gokz_client: gokz_rs::Client,
	pub global_maps: Vec<GlobalMap>,
	pub database_connection: Pool<Postgres>,
}

impl State {
	pub async fn new(config: Config) -> Self {
		let database_connection = PgPoolOptions::new()
			.min_connections(5)
			.max_connections(20)
			.connect(&config.database_url)
			.await
			.expect("Failed to establish database connection.");

		let token_storage = TokenStore {
			client_id: config.client_id.clone(),
			client_secret: config.client_secret.clone(),
			credentials_table: config.credentials_table.clone(),
			database_connection: database_connection.clone(),
		};

		let login_credentials = RefreshingLoginCredentials::init(
			config.client_id.clone(),
			config.client_secret.clone(),
			token_storage,
		);

		let client_config = ClientConfig::new_simple(login_credentials);

		let (message_stream, twitch_client) = TwitchClient::new(client_config);

		let gokz_client = gokz_rs::Client::new();

		let global_maps = schnosebot::global_map::GlobalMap::fetch(true, &gokz_client)
			.await
			.expect("Failed to fetch global maps.");

		Self {
			config,
			twitch_client,
			message_stream,
			gokz_client,
			global_maps,
			database_connection,
		}
	}

	pub async fn run(mut self) -> Result<()> {
		#[derive(sqlx::FromRow)]
		struct ChannelName(String);

		let channel_names = sqlx::query_as::<_, ChannelName>(&format!(
			"SELECT channel_name FROM {}",
			&self.config.streamers_table
		))
		.fetch_all(&self.database_connection)
		.await
		.expect("Failed to fetch channel names.")
		.into_iter()
		.map(|channel_name| channel_name.0)
		.collect::<Vec<_>>();

		for channel in channel_names {
			info!("Joining `{channel}`.");
			if let Err(why) = self.twitch_client.join(channel) {
				error!("Failed joining channel: {why:?}");
			}
		}

		while let Some(message) = self.message_stream.recv().await {
			info!("Received message");

			if let ServerMessage::Privmsg(message) = message {
				let channel = message.channel_login;
				let user = message.sender.name;
				let message = message.message_text;

				info!("[{channel}] {user}: {message}");
			}
		}

		Ok(())
	}
}
