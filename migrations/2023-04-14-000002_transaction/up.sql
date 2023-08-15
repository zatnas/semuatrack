-- Your SQL goes here
CREATE TABLE "transaction" (
	id BIGSERIAL PRIMARY KEY,
	account_id INT NOT NULL DEFAULT 0,
	category_id SMALLINT NOT NULL DEFAULT 0,
	datetime BIGINT NOT NULL CHECK (datetime > 0),
	amount NUMERIC(50, 2) NOT NULL,
	note TEXT,
	place TEXT,
	CONSTRAINT transaction_category FOREIGN KEY (category_id) REFERENCES category(id),
	CONSTRAINT transaction_account FOREIGN KEY (account_id) REFERENCES account(id)
);

INSERT INTO "transaction" (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-10 20:30:12+08')),
	100.50,
	'Transaction_Note_1',
	'Transaction_Place_1'
);

INSERT INTO "transaction" (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-20 10:10:12+08')),
	25.75,
	'Transaction_Note_2',
	'Transaction_Place_2'
);

INSERT INTO "transaction" (datetime, amount, note, place) VALUES (
	(SELECT EXTRACT(EPOCH FROM TIMESTAMP WITH TIME ZONE '2022-10-30 13:15:12+08')),
	11.55,
	'Transaction_Note_3',
	'Transaction_Place_3'
);
