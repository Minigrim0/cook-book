-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_recipe_images_recipe_id;
DROP INDEX IF EXISTS idx_recipe_images_image_id;

DROP TABLE IF EXISTS recipe_images;
DROP TABLE IF EXISTS image_blobs;

ALTER TABLE recipe ADD COLUMN image VARCHAR;
