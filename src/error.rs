#![allow(clippy::upper_case_acronyms)]

use {
	gokz_rs::{MapIdentifier, Mode, PlayerIdentifier},
	std::fmt::Display,
	thiserror::Error,
	tracing::{debug, error},
};

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! yeet {
	($e:expr) => {
		return Err($e);
	};
}

pub(crate) use yeet;

#[derive(Debug, Clone, Error)]
pub enum Error {
	#[error("Unknown error occurred.")]
	Unknown,

	#[error("{0}")]
	Custom(String),

	#[error("{error}")]
	GOKZ { error: gokz_rs::Error },

	#[error("Failed to access database.")]
	DatabaseAccess,

	#[error("Incorrect arguments. Expected {expected}")]
	IncorrectArgs { expected: String },

	#[error("No data about streamer found. Please supply arguments ({arg}).")]
	NoDataAboutStreamer { arg: String },

	#[error("")]
	NotACommand,

	// #[error("Unknown command `{0}`")]
	#[error("")]
	UnknownCommand(String),

	#[error("`{map}` is not a global map.")]
	MapNotGlobal { map: String },

	#[error("Twitch API Error.")]
	Twitch,
}

impl From<gokz_rs::Error> for Error {
	fn from(error: gokz_rs::Error) -> Self {
		error!("GOKZ Error.");
		debug!("{error:?}");
		Self::GOKZ { error }
	}
}

#[allow(clippy::cognitive_complexity)] // ???
impl From<sqlx::Error> for Error {
	fn from(error: sqlx::Error) -> Self {
		error!("SQLx Error.");
		error!("{error:?}");

		match error {
			sqlx::Error::Database(why) => {
				debug!("{why:?}");
				Self::DatabaseAccess
			}
			why => {
				debug!("{why:?}");
				Self::Unknown
			}
		}
	}
}

impl From<twitch_irc::validate::Error> for Error {
	fn from(error: twitch_irc::validate::Error) -> Self {
		error!("Error talking to Twitch: {error:?}");
		Self::Twitch
	}
}

pub trait GenParseError {
	fn incorrect() -> Error;
	fn no_data(arg: impl Display) -> Error {
		Error::NoDataAboutStreamer { arg: arg.to_string() }
	}
}

macro_rules! gen_parse_err {
	($t:ty, $incorrect:expr) => {
		impl GenParseError for $t {
			fn incorrect() -> Error {
				$incorrect
			}
		}
	};
}

pub(crate) use gen_parse_err;

gen_parse_err!(Mode, Error::IncorrectArgs { expected: String::from("mode") });
gen_parse_err!(PlayerIdentifier, Error::IncorrectArgs { expected: String::from("player") });
gen_parse_err!(MapIdentifier, Error::IncorrectArgs { expected: String::from("map") });
gen_parse_err!(u32, Error::IncorrectArgs { expected: String::from("RecordID") });
