-- Your SQL goes here
-- Create a new table to store image blobs
CREATE TABLE image_blobs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    image_data BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    hash TEXT UNIQUE NOT NULL -- Store a hash of the image data for deduplication
);

-- Create a junction table for many-to-many relationship between recipes and images
CREATE TABLE recipe_images (
    recipe_id INTEGER NOT NULL,
    image_id INTEGER NOT NULL,
    PRIMARY KEY (recipe_id, image_id),
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES image_blobs(id) ON DELETE CASCADE
);

-- Remove the image_url column from the recipes table
ALTER TABLE recipe DROP COLUMN image;

-- Add indexes to improve query performance
CREATE INDEX idx_recipe_images_recipe_id ON recipe_images(recipe_id);
CREATE INDEX idx_recipe_images_image_id ON recipe_images(image_id);

-- SQLite doesn't support user-defined functions in SQL, so we'll need to handle
-- hash calculation in the application code when inserting new images.

-- Note: Triggers in SQLite are more limited than in PostgreSQL.
-- The hash calculation should be done in the application code before inserting.

