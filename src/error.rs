use {
	thiserror::Error,
	tracing::{debug, error},
};

pub type Result<T> = std::result::Result<T, Error>;

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

	#[error("No database entries found.")]
	NoDatabaseEntries,
}

impl From<gokz_rs::Error> for Error {
	fn from(error: gokz_rs::Error) -> Self {
		error!("GOKZ Error.");
		debug!("{error:?}");
		Self::GOKZ { error }
	}
}

impl From<sqlx::Error> for Error {
	fn from(error: sqlx::Error) -> Self {
		error!("SQLx Error.");
		error!("{error:?}");

		match error {
			sqlx::Error::Database(why) => {
				debug!("{why:?}");
				Self::DatabaseAccess
			}
			sqlx::Error::RowNotFound => Self::NoDatabaseEntries,
			why => {
				debug!("{why:?}");
				Self::Unknown
			}
		}
	}
}
