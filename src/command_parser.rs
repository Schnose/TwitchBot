use {
	crate::{
		error::{yeet, Error, Result},
		state::State,
	},
	tracing::info,
	twitch_irc::message::PrivmsgMessage,
};

#[derive(Debug, Clone)]
pub enum Command {
	Ping,
	APIStatus,
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

		match command_name.as_str() {
			"ping" => Ok(Self::Ping),
			"api" | "apistatus" => Ok(Self::APIStatus),
			cmd => Err(Error::UnknownCommand(cmd.to_owned())),
		}
	}
}
