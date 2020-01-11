CREATE TABLE guilds (
	id BIGINT PRIMARY KEY,
	cache_name VARCHAR(255) NOT NULL,
	online BOOLEAN NOT NULL,
	listed BOOLEAN NOT NULL
);

CREATE INDEX ON guilds (online, listed, cache_name);

CREATE TABLE channels (
	id BIGINT PRIMARY KEY,
	guild_id BIGINT NOT NULL,
	cache_name VARCHAR(255) NOT NULL,
	cache_desc TEXT NOT NULL,
	FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);

CREATE INDEX ON channels (guild_id, cache_name);

CREATE TABLE channel_hours (
	channel_id BIGINT,
	date DATE,
	hour INT,
	message BIGINT NOT NULL,
	PRIMARY KEY (channel_id, date, hour),
	FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
);

CREATE TABLE known_invites (
	code VARCHAR(255) PRIMARY KEY,
	guild_id BIGINT NOT NULL,
	FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
);
