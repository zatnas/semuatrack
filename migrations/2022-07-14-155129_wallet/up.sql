CREATE TABLE 'transactions' (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	datetime INTEGER NOT NULL,
	amount REAL NOT NULL CHECK(ROUND(amount, 2) == amount),
	note TEXT,
	place TEXT
);

INSERT INTO 'transactions' (datetime, amount, note, place) VALUES (
	CAST(strftime('%s', '2022-10-10 20:30:12') as integer),
	100.50,
	'Transaction_Note_1',
	'Transaction_Place_1'
);

INSERT INTO 'transactions' (datetime, amount, note, place) VALUES (
	CAST(strftime('%s', '2022-10-20 10:10:12') as integer),
	25.75,
	'Transaction_Note_2',
	'Transaction_Place_2'
);

INSERT INTO 'transactions' (datetime, amount, note, place) VALUES (
	CAST(strftime('%s', '2022-10-30 13:15:12') as integer),
	11.55,
	'Transaction_Note_3',
	'Transaction_Place_3'
);

