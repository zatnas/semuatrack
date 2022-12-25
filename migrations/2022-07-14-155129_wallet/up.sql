CREATE TABLE account (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	color TEXT NOT NULL
);

CREATE TABLE category (
	id SMALLSERIAL PRIMARY KEY,
	parent_id SMALLINT,
	name TEXT NOT NULL UNIQUE,
	color TEXT,
	icon TEXT
);
ALTER TABLE category ADD CONSTRAINT category_category FOREIGN KEY (parent_id) REFERENCES category(id);

CREATE TABLE tag (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	color TEXT NOT NULL
);

CREATE TABLE cashflow (
	id BIGSERIAL PRIMARY KEY,
	account_id INT,
	category_id SMALLINT,
	datetime BIGINT NOT NULL CHECK (datetime > 0),
	amount NUMERIC(50, 2) NOT NULL,
	note TEXT,
	place TEXT,
	CONSTRAINT cashflow_category FOREIGN KEY (category_id) REFERENCES category(id),
	CONSTRAINT cashflow_account FOREIGN KEY (account_id) REFERENCES account(id)
);

INSERT INTO cashflow (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-10 20:30:12+08')),
	100.50,
	'Transaction_Note_1',
	'Transaction_Place_1'
);

INSERT INTO cashflow (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-20 10:10:12+08')),
	25.75,
	'Transaction_Note_2',
	'Transaction_Place_2'
);

INSERT INTO cashflow (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-30 13:15:12+08')),
	11.55,
	'Transaction_Note_3',
	'Transaction_Place_3'
);

