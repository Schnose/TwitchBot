use {
	crate::{
		error::{yeet, Error, Result},
		parsing::{parse_args, Parsable},
		state::State,
	},
	gokz_rs::{MapIdentifier, Mode, PlayerIdentifier},
	schnosebot::global_map::GlobalMap,
	twitch_irc::message::PrivmsgMessage,
};

#[derive(Debug, Clone)]
pub enum Command {
	Ping,
	APIStatus,
	BPB {
		map: GlobalMap,
		course: u8,
		mode: Mode,
		player: PlayerIdentifier,
	},
	BWR {
		map: GlobalMap,
		course: u8,
		mode: Mode,
	},
	Map {
		map: GlobalMap,
	},
	MostRecentRun,
	PB {
		map: GlobalMap,
		mode: Mode,
		player: PlayerIdentifier,
	},
	Player {
		player: PlayerIdentifier,
	},
	Recent {
		player: PlayerIdentifier,
	},
	Record {
		record_id: u32,
	},
	WR {
		map: GlobalMap,
		mode: Mode,
	},
}

impl Command {
	pub async fn parse(state: &State, message: &PrivmsgMessage) -> Result<Self> {
		if !message
			.message_text
			.starts_with(&state.config.command_prefix)
		{
			yeet!(Error::NotACommand);
		}

		let trimmed = message.message_text.trim();

		let (_, args) = trimmed
			.split_once(&state.config.command_prefix)
			.unwrap();

		let mut args = args.split(' ');

		let command_name = args.next().unwrap().to_lowercase();

		let args = args
			.filter(|s| !s.is_empty())
			.collect::<Vec<_>>();

		let args = args.join(" ");

		let channel_id = message
			.channel_id
			.parse::<i32>()
			.expect("ChannelID should always be a number.");

		match command_name.as_str() {
			"ping" => Ok(Self::Ping),
			"api" | "apistatus" => Ok(Self::APIStatus),
			"bpb" => {
				let (map, course, mode, player) = parse_args!(
					args, "opt" MapIdentifier, "opt" u8, "opt" Mode, "opt" PlayerIdentifier
				)?;

				let map = map.parse(channel_id, state).await?;
				let map = GlobalMap::fuzzy_search(&state.global_maps, map.clone())
					.ok_or(Error::MapNotGlobal { map: map.to_string() })?;

				let course = course.unwrap_or(1).max(1);

				let mode = mode
					.parse(channel_id, state)
					.await
					.unwrap_or(Mode::KZTimer);

				let player = player.parse(channel_id, state).await?;

				Ok(Self::BPB { map, course, mode, player })
			}
			"bwr" => {
				let (map, course, mode) = parse_args!(
					args, "opt" MapIdentifier, "opt" u8, "opt" Mode
				)?;

				let map = map.parse(channel_id, state).await?;
				let map = GlobalMap::fuzzy_search(&state.global_maps, map.clone())
					.ok_or(Error::MapNotGlobal { map: map.to_string() })?;

				let course = course.unwrap_or(1).max(1);

				let mode = mode
					.parse(channel_id, state)
					.await
					.unwrap_or(Mode::KZTimer);

				Ok(Self::BWR { map, course, mode })
			}
			"m" | "map" => {
				let map = parse_args!(args, MapIdentifier)?;
				let map = GlobalMap::fuzzy_search(&state.global_maps, map.clone())
					.ok_or(Error::MapNotGlobal { map: map.to_string() })?;

				Ok(Self::Map { map })
			}
			"mrr" | "mostrecentrecord" => Ok(Self::MostRecentRun),
			"pb" => {
				let (map, mode, player) = parse_args!(
					args, "opt" MapIdentifier, "opt" Mode, "opt" PlayerIdentifier
				)?;

				let map = map.parse(channel_id, state).await?;
				let map = GlobalMap::fuzzy_search(&state.global_maps, map.clone())
					.ok_or(Error::MapNotGlobal { map: map.to_string() })?;

				let mode = mode
					.parse(channel_id, state)
					.await
					.unwrap_or(Mode::KZTimer);

				let player = player.parse(channel_id, state).await?;

				Ok(Self::PB { map, mode, player })
			}
			"p" | "player" => {
				let player = parse_args!(
					args, "opt" PlayerIdentifier
				)?;

				let player = player.parse(channel_id, state).await?;

				Ok(Self::Player { player })
			}
			"recent" => {
				let player = parse_args!(
					args, "opt" PlayerIdentifier
				)?;

				let player = player.parse(channel_id, state).await?;

				Ok(Self::Recent { player })
			}
			"r" | "record" => {
				let record_id = parse_args!(args, u32)?;

				Ok(Self::Record { record_id })
			}
			"wr" => {
				let (map, mode) = parse_args!(
					args, "opt" MapIdentifier, "opt" Mode
				)?;

				let map = map.parse(channel_id, state).await?;
				let map = GlobalMap::fuzzy_search(&state.global_maps, map.clone())
					.ok_or(Error::MapNotGlobal { map: map.to_string() })?;

				let mode = mode
					.parse(channel_id, state)
					.await
					.unwrap_or(Mode::KZTimer);

				Ok(Self::WR { map, mode })
			}
			cmd => Err(Error::UnknownCommand(cmd.to_owned())),
		}
	}
}
