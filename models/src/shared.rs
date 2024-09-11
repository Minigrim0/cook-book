use serde::{Deserialize, Serialize};

use crate::models::Step;

use super::models::{Author, Category, Cuisine, Ingredient, Rating, Recipe, Unit};

#[derive(Serialize, Deserialize, Debug, Default)]
/// Include metadata on recipes, amount of recipe, amout of cuisines, ...
pub struct RecipeMeta {
    pub recipe_amount: i32,
    pub cuisine_amount: i32,
    pub ingredients_amount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// An ingredient in a recipe, contains the quantity,
/// condition & unit.
pub struct CompleteIngredient {
    pub id: i32,
    pub ingredient: Ingredient,
    pub ingredient_image: Option<String>,
    pub unit: Unit,
    pub amount: f32,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Model shared to the frontend to display a recipe.
/// Loading this model involves multiple other models.
pub struct CompleteRecipe {
    pub id: i32,
    pub name: String,
    pub cook_time: i32,
    pub prep_time: i32,
    pub yield_: i32,
    pub author: Option<Author>,
    pub rating: Option<Rating>,
    pub category: Option<Category>,
    pub image: Option<String>,
    pub ingredients: Vec<Result<CompleteIngredient, String>>,
    pub steps: Vec<Step>,
}

/// The type of the shared data for the ingredients
/// This is a tuple of the ingredients (paginated), the total number of ingredients and the number of pages
/// corresponding to the query
pub type PaginatedIngredients = (Vec<Ingredient>, usize, usize);
pub type PaginatedRecipe = (Vec<Recipe>, usize, usize);
pub type PaginatedCuisine = (Vec<Cuisine>, usize, usize);
