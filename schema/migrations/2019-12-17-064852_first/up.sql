CREATE TABLE guild (
	id BIGINT PRIMARY KEY,
	cache_name VARCHAR(255),
	listed BOOLEAN
);

CREATE INDEX ON guild (listed, cache_name);

CREATE TABLE channel (
	id BIGINT PRIMARY KEY,
	guild BIGINT,
	cache_name VARCHAR(255),
	cache_desc TEXT,
	FOREIGN KEY (guild) REFERENCES guild(id) ON DELETE CASCADE
);

CREATE INDEX ON channel (guild, cache_name);

CREATE TABLE channel_day (
	channel BIGINT,
	date DATE,
	message BIGINT,
	PRIMARY KEY (channel, date),
	FOREIGN KEY (channel) REFERENCES channel(id) ON DELETE CASCADE
);

CREATE TABLE known_invite (
	code VARCHAR(255) PRIMARY KEY,
	guild BIGINT,
	FOREIGN KEY (guild) REFERENCES guild(id) ON DELETE CASCADE
);
