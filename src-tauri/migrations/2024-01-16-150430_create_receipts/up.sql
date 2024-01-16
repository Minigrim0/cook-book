-- Your SQL goes here
pragma foreign_keys=on;

CREATE TABLE author (
    id INTEGER PRIMARY KEY,
    type VARCHAR NOT NULL,  -- Person or Organization
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL UNIQUE  -- URL to the author's profile
);

CREATE TABLE rating (
    id INTEGER PRIMARY KEY,
    rating INTEGER NOT NULL,  -- Average rating
    amount INTEGER NOT NULL  -- Amount of votes
);

CREATE TABLE unit (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);

INSERT INTO unit (name) VALUES 
    ('g'),
    ('kg'),
    ('oz'),
    ('lb'),
    ('l'),
    ('dl'),
    ('tblsp'),
    ('tsp'),
    ('ml'),
    ('cup');

CREATE TABLE ingredient (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE receipt_ingredient (
    id INTEGER PRIMARY KEY,
    receiptId INTEGER NOT NULL REFERENCES receipt(id),
    ingredientId INTEGER NOT NULL REFERENCES ingredient(id),
    unitId INTEGER NOT NULL REFERENCES unit(id),
    amount VARCHAR NOT NULL
);

CREATE TABLE step (
    id INTEGER PRIMARY KEY,
    receiptId INTEGER NOT NULL REFERENCES receipt(id),
    step INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE category (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE cuisine (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE receipt_cuisine (
    id INTEGER PRIMARY KEY,
    receiptId INTEGER NOT NULL REFERENCES receipt(id),
    cuisineId INTEGER NOT NULL REFERENCES cuisine(id)
);

CREATE TABLE receipt (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    cookTime INTEGER NOT NULL,  -- Time to cook in seconds
    prepTime INTEGER NOT NULL,  -- Time to prepare in seconds
    yield INTEGER NOT NULL,  -- Yield of the receipt
    authorId INTEGER NOT NULL REFERENCES author(id),
    ratingId INTEGER NOT NULL REFERENCES rating(id),
    categoryId INTEGER NOT NULL REFERENCES category(id)
);
