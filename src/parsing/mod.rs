use {
	crate::{
		error::{Error, GenParseError, Result},
		state::State,
	},
	gokz_rs::{MapIdentifier, Mode, PlayerIdentifier, SteamID},
	shuttle_runtime::async_trait,
};

mod parse_args;
pub(crate) use parse_args::parse_args;

#[async_trait]
pub trait Parsable {
	type Output;

	async fn parse(self, channel_id: i32, state: &State) -> Result<Self::Output>;
}

#[async_trait]
impl Parsable for Option<MapIdentifier> {
	type Output = MapIdentifier;

	async fn parse(self, channel_id: i32, state: &State) -> Result<Self::Output> {
		if let Some(map_identifier) = self {
			return Ok(map_identifier);
		}

		state
			.fetch_streamer(channel_id)
			.await?
			.map_name
			.ok_or(MapIdentifier::no_data("map"))
			.map(Into::into)
	}
}

#[async_trait]
impl Parsable for Option<Mode> {
	type Output = Mode;

	async fn parse(self, channel_id: i32, state: &State) -> Result<Self::Output> {
		if let Some(mode) = self {
			return Ok(mode);
		}

		match state
			.fetch_streamer(channel_id)
			.await?
			.mode
		{
			Some(mode_id) => Ok(Mode::try_from(mode_id as u8)?),
			None => Err(MapIdentifier::no_data("mode")),
		}
	}
}

#[async_trait]
impl Parsable for Option<PlayerIdentifier> {
	type Output = PlayerIdentifier;

	async fn parse(self, channel_id: i32, state: &State) -> Result<Self::Output> {
		if let Some(player_identifier) = self {
			return Ok(player_identifier);
		}

		let streamer_info = state.fetch_streamer(channel_id).await?;

		match streamer_info.steam_id {
			Some(id32) => {
				let id32 = u32::try_from(id32)
					.map_err(|_| Error::Custom(String::from("Invalid SteamID in database.")))?;

				let steam_id = SteamID::from_id32(id32);
				Ok(steam_id.into())
			}
			None => streamer_info
				.player_name
				.ok_or(MapIdentifier::no_data("player"))
				.map(Into::into),
		}
	}
}
