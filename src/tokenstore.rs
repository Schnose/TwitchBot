use {
	crate::{database::CredentialsRow, error::Error},
	shuttle_runtime::async_trait,
	sqlx::{Pool, Postgres},
	twitch_irc::login::{TokenStorage, UserAccessToken},
};

#[derive(Debug)]
pub struct TokenStore {
	pub client_id: String,
	pub client_secret: String,
	pub credentials_table: String,
	pub database_connection: Pool<Postgres>,
}

#[async_trait]
impl TokenStorage for TokenStore {
	type LoadError = Error;
	type UpdateError = Error;

	async fn load_token(&mut self) -> std::result::Result<UserAccessToken, Self::LoadError> {
		let latest_config: CredentialsRow = sqlx::query_as(
			r#"
			SELECT * FROM credentials
			ORDER BY created_on DESC
			LIMIT 1
			"#,
		)
		.fetch_one(&self.database_connection)
		.await?;

		Ok(UserAccessToken {
			access_token: latest_config.access_token,
			refresh_token: latest_config.refresh_token,
			created_at: latest_config.created_on,
			expires_at: Some(latest_config.expires_on),
		})
	}

	async fn update_token(
		&mut self,
		token: &UserAccessToken,
	) -> std::result::Result<(), Self::UpdateError> {
		let credentials_table = &self.credentials_table;

		sqlx::query(&format!(
			r#"
			UPDATE {credentials_table}
			SET
			  access_token = $1,
			  refresh_token = $2,
			  created_on = $3,
			  expires_on = $4
			"#
		))
		.bind(&token.access_token)
		.bind(&token.refresh_token)
		.bind(token.created_at)
		.bind(token.expires_at)
		.execute(&self.database_connection)
		.await?;

		Ok(())
	}
}
