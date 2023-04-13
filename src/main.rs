//! This is a twitch bot for CS:GO KZ.
//!
//! It talks to different APIs and its own Database to provide useful functionality that you would
//! otherwise only get ingame or from a website like [KZ:GO](https://kzgo.eu). This functionality
//! mostly revolves around `/` commands, including:
//! - `/pb`
//! - `/wr`
//! - `/recent`
//! - `/player`
//!
//! and many more! For a full list check out the [Wiki](https://github.com/Schnose/TwitchBot/wiki).
//! I am running a public instance that you can invite to your channel by visiting [the account's
//! Twitch Channel](https://twitch.tv/SchnoseBot) and typing `!join` into the chat!

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::style, clippy::complexity, clippy::cognitive_complexity)]
#![deny(clippy::perf, clippy::correctness)]

use {
	crate::{config::Config, shuttle_integration::ShuttleResult, state::State},
	shuttle_secrets::SecretStore,
};

mod command_parser;
mod commands;
mod config;
mod database;
mod error;
mod parsing;
mod shuttle_integration;
mod state;
mod tokenstore;

#[shuttle_runtime::main]
async fn schnosebot(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleResult {
	let config = Config::new(&secret_store);
	let state = State::new(config).await;
	Ok(state)
}
