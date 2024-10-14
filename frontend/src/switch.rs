use yew::{html, Html};
use yew_router::prelude::Switch;

use super::pages::{
    admin_pages, DefaultPage, IngredientsPage, LoaderPage, RecipeDetailsPage, RecipesPage,
    TimerPage,
};
use super::routes::{AdminRoute, RecipeRoute, Route, ToolsRoute};
use super::AppProps;

/// Switch for tools routes
fn switch_tools(route: ToolsRoute) -> Html {
    match route {
        ToolsRoute::Load => html! { <LoaderPage /> },
        ToolsRoute::CreateRecipe => html! { <p>{"Create recipe"}</p> },
        ToolsRoute::DuplicateFinder => html! { <p>{"Duplicate finder"}</p> },
    }
}

/// Switch for recipe routes
fn switch_recipe(route: RecipeRoute) -> Html {
    match route {
        RecipeRoute::RecipeRoot => html! { <RecipesPage /> },
        RecipeRoute::ByCuisine => html! { <DefaultPage /> },
        RecipeRoute::FromIngredients => html! { <IngredientsPage /> },
        RecipeRoute::RecipeDetails { id } => html! { <RecipeDetailsPage recipe_id={id} /> },
    }
}

/// Switch for admin routes
fn switch_admin(route: AdminRoute) -> Html {
    match route {
        AdminRoute::AdminRoot => html! { <admin_pages::AdminRoot /> },
        AdminRoute::Ingredients => html! { <admin_pages::AdminIngredientList /> },
        AdminRoute::Recipes => html! { <admin_pages::AdminRecipeList /> },
        AdminRoute::Units => html! { <admin_pages::AdminUnitList /> },
    }
}

/// Switch for all routes
pub fn switch(route: Route, props: AppProps) -> Html {
    match route {
        Route::Home => html! { <RecipesPage /> },
        Route::ToolsRoot | Route::Tools => {
            html! { <Switch<ToolsRoute> render={switch_tools} /> }
        }
        Route::RecipeRoot | Route::Recipe => {
            html! { <Switch<RecipeRoute> render={switch_recipe} /> }
        }
        Route::Timers => html! { <TimerPage
            timers={props.timers}
            timer_add_callback={props.timer_add_callback}
            timer_remove_callback={props.timer_remove_callback}
            timer_update_callback={props.timer_update_callback}
            timer_stop_callback={props.timer_stop_callback}
        /> },
        Route::Converters => html! { <p>{"Converters"}</p> },
        Route::NotFound => {
            html! { <div class="position-absolute top-50 start-50 translate-middle"><h1>{"Not found"}</h1></div> }
        }
        Route::AdminRoot | Route::Admin => {
            html! { <Switch<AdminRoute> render={switch_admin} /> }
        }
    }
}
