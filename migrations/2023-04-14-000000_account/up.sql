-- Your SQL goes here
CREATE TABLE account (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	color TEXT NOT NULL
);

INSERT INTO account (id, name, color) VALUES (
	0, 'Default', 'Default'
);
