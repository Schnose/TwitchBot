use sqlx::{
	types::{
		chrono::{DateTime, Utc},
		Uuid,
	},
	FromRow,
};

#[derive(Debug, FromRow)]
pub struct CredentialsRow {
	pub client_id: String,
	pub client_secret: String,
	pub access_token: String,
	pub refresh_token: String,
	pub created_on: DateTime<Utc>,
	pub expires_on: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct StreamerRow {
	pub channel_id: i32,
	pub channel_name: String,
	pub api_key: Option<Uuid>,
}
