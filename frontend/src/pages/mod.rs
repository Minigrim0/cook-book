mod admin;
mod default;
mod ingredients_list;
mod loader;
mod recipe_details;
mod recipes_list;
mod services;
mod timer;

pub use default::DefaultPage;
pub use ingredients_list::IngredientsPage;
pub use loader::LoaderPage;
pub use recipe_details::RecipeDetailsPage;
pub use recipes_list::RecipesPage;
pub use timer::TimerPage;
pub mod admin_pages {
    pub use super::admin::AdminIngredientList;
    pub use super::admin::AdminRecipeList;
    pub use super::admin::AdminRoot;
    pub use super::admin::AdminUnitList;
}
