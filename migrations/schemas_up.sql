CREATE TABLE IF NOT EXISTS credentials (
	client_id VARCHAR(255) NOT NULL,
	client_secret VARCHAR(255) NOT NULL,
	access_token VARCHAR(255) NOT NULL,
	refresh_token VARCHAR(255) NOT NULL,
	created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	expires_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

	PRIMARY KEY (client_id)
);

CREATE TABLE IF NOT EXISTS streamers (
	channel_id INT NOT NULL,
	channel_name VARCHAR(255) NOT NULL,
	api_key UUID,

	player_name VARCHAR(255),
	steam_id INT,
	map_name VARCHAR(255),
	mode INT2,

	PRIMARY KEY(channel_id)
);
