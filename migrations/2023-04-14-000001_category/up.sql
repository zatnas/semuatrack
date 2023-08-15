-- Your SQL goes here
CREATE TABLE category (
	id SMALLSERIAL PRIMARY KEY,
	parent_id SMALLINT,
	name TEXT NOT NULL UNIQUE,
	color TEXT,
	icon TEXT
);
ALTER TABLE category
ADD CONSTRAINT category_category
FOREIGN KEY (parent_id) REFERENCES category(id);

INSERT INTO category (id, name) VALUES (
0,
'Default'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Beverages'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Beverages'),
'Cafe'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Beverages'),
'Groceries'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Beverages'),
'Restaurant'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Beverages'),
'Fast Food'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Shopping'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Apparel'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Medicine'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Electronics'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Free Time'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Gifts'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Health & Beauty'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Home'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Jewels'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Pets'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Kids'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Shopping'),
'Tools'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Housing'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Utilities'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Maintenance'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Mortgage'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Property Insurance'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Rent'
);

INSERT INTO category (parent_id, name) VALUES (
(SELECT id FROM category WHERE name = 'Housing'),
'Services'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Transportation'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Vehicle'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Life & Entertainment'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Communication, PC'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Financial Expenses'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Investments'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Income'
);

INSERT INTO category (parent_id, name) VALUES (
NULL,
'Others'
);
