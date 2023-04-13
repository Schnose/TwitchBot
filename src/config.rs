use shuttle_secrets::SecretStore;

#[derive(Debug, Clone)]
pub struct Config {
	/// `ClientID` of the bot's Twitch account
	pub client_id: String,

	/// Client Secret of the bot's Twitch account
	pub client_secret: String,

	/// PostgreSQL connection string
	pub database_url: String,

	/// PostgreSQL table name to store credentials for the bot
	pub credentials_table: String,

	/// PostgreSQL table name to store streamer information in
	pub streamers_table: String,
}

impl Config {
	pub fn new(secret_store: &SecretStore) -> Self {
		Self {
			client_id: secret_store
				.get("CLIENT_ID")
				.expect("Missing `CLIENT_ID` secret."),
			client_secret: secret_store
				.get("CLIENT_SECRET")
				.expect("Missing `CLIENT_SECRET` secret."),
			database_url: secret_store
				.get("DATABASE_URL")
				.expect("Missing `DATABASE_URL` secret."),
			credentials_table: secret_store
				.get("CREDENTIALS_TABLE")
				.expect("Missing `CREDENTIALS_TABLE` secret."),
			streamers_table: secret_store
				.get("STREAMERS_TABLE")
				.expect("Missing `STREAMERS_TABLE` secret."),
		}
	}
}
