use {
	axum::{
		extract::{Json, State},
		http::{HeaderMap, StatusCode},
		routing::{get, post},
		Router, Server,
	},
	gokz_rs::{Mode, SteamID},
	serde::Deserialize,
	sqlx::{types::Uuid, FromRow, Pool, Postgres},
	std::net::SocketAddr,
	tracing::{error, warn},
};

#[derive(Debug, Clone)]
struct Database {
	streamers_table: String,
	database_connection: Pool<Postgres>,
}

pub async fn run(addr: SocketAddr, streamers_table: String, database_connection: Pool<Postgres>) {
	let database = Database { streamers_table, database_connection };

	let router = Router::new()
		.route("/", get(|| async { "(͡ ͡° ͜ つ ͡͡°)" }))
		.route("/health", get(|| async { "(͡ ͡° ͜ つ ͡͡°)" }))
		.route("/streamer", post(post_info))
		.with_state(database);

	Server::bind(&addr)
		.serve(router.into_make_service())
		.await
		.expect("Failed to run axum server.");
}

#[derive(Debug, Deserialize)]
struct Params {
	player_name: Option<String>,
	steam_id: Option<SteamID>,
	map_name: Option<String>,
	mode: Option<Mode>,
}

#[derive(Debug, PartialEq, FromRow)]
struct ApiKey(Uuid);

#[axum::debug_handler]
async fn post_info(
	headers: HeaderMap,
	State(Database { streamers_table, database_connection }): State<Database>,
	Json(params): Json<Params>,
) -> StatusCode {
	let Some(api_key) = headers.get("x-schnose-api-key") else {
		warn!("Got a request but without an API key.");
		return StatusCode::BAD_REQUEST;
	};

	let Ok(api_key) = api_key.to_str() else {
		warn!("Got a request but API key was not a valid string.");
		return StatusCode::BAD_REQUEST;
	};

	let Ok(api_key) = api_key.parse::<Uuid>() else {
		warn!("Got a request but API key was not a valid Uuid.");
		return StatusCode::BAD_REQUEST;
	};

	let Ok(api_keys) = sqlx::query_as::<_, ApiKey>("SELECT api_key FROM streamers")
		.fetch_all(&database_connection)
		.await else {
			error!("Failed to fetch API keys.");
			return StatusCode::INTERNAL_SERVER_ERROR;
		};

	if !api_keys.contains(&ApiKey(api_key)) {
		error!("User provided invalid API key.");
		return StatusCode::UNAUTHORIZED;
	}

	if let Err(why) = sqlx::query(&format!(
		r#"
		UPDATE {streamers_table}
		SET
		  player_name = $1,
		  steam_id = $2,
		  map_name = $3,
		  mode = $4
		WHERE api_key = $5
		"#
	))
	.bind(params.player_name)
	.bind(
		params
			.steam_id
			.map(|steam_id| steam_id.as_id32() as i32),
	)
	.bind(params.map_name)
	.bind(params.mode.map(|mode| mode as i16))
	.bind(api_key)
	.execute(&database_connection)
	.await
	{
		error!("Failed to update database: {why:?}");
		return StatusCode::INTERNAL_SERVER_ERROR;
	}

	StatusCode::OK
}
