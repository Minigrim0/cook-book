-- Your SQL goes here
pragma foreign_keys=on;

CREATE TABLE author (
    id INTEGER NOT NULL PRIMARY KEY,
    type VARCHAR NOT NULL,  -- Person or Organization
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL UNIQUE  -- URL to the author's profile
);

CREATE TABLE rating (
    id INTEGER NOT NULL PRIMARY KEY,
    score FLOAT NOT NULL,  -- Average rating
    amount INTEGER NOT NULL  -- Amount of votes
);

CREATE TABLE unit (
    id INTEGER NOT NULL PRIMARY KEY,
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
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE recipe_ingredient (
    id INTEGER NOT NULL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipe(id),
    ingredient_id INTEGER NOT NULL REFERENCES ingredient(id),
    unit_id INTEGER NOT NULL REFERENCES unit(id),
    amount VARCHAR NOT NULL
);

CREATE TABLE step (
    id INTEGER NOT NULL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipe(id),
    number INTEGER NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE category (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE cuisine (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE recipe_cuisine (
    id INTEGER NOT NULL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipe(id),
    cuisine_id INTEGER NOT NULL REFERENCES cuisine(id)
);

CREATE TABLE recipe (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    cook_time INTEGER NOT NULL,  -- Time to cook in seconds
    prep_time INTEGER NOT NULL,  -- Time to prepare in seconds
    yield INTEGER NOT NULL,  -- Yield of the recipe
    author_id INTEGER NOT NULL REFERENCES author(id),
    rating_id INTEGER NOT NULL REFERENCES rating(id),
    category_id INTEGER NOT NULL REFERENCES category(id)
);
