use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/timers")]
    Timers,
    #[at("/converter")]
    Converters,
    #[at("/tools")]
    ToolsRoot,
    #[at("/tools/*")]
    Tools,
    #[at("/recipe")]
    RecipeRoot,
    #[at("/recipe/*")]
    Recipe,
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum ToolsRoute {
    #[at("/tools/load")]
    Load,
    #[at("/tools/create")]
    CreateRecipe,
    #[at("/tools/dpfinder")]
    DuplicateFinder,
}

#[derive(Clone, Routable, PartialEq)]
pub enum RecipeRoute {
    #[at("/recipe/")]
    RecipeRoot,
    #[at("/recipe/by-cuisine")]
    ByCuisine,
    #[at("/recipe/from-ingredient")]
    FromIngredients,
    #[at("/recipe/:id/details")]
    RecipeDetails { id: i32 },
}


#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/ingredients")]
    Ingredients,
    #[at("/admin/recipes")]
    Recipes,
    #[at("/admin/units")]
    Units,
}
