use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;

use components::{FooterComponent, HeaderComponent, SidebarComponent};
use pages::{DefaultPage, LoaderPage, TimerPage};
use routes::{Route, ToolsRoute, RecipeRoute};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path() -> Result<JsValue, JsValue>;
}

fn switch_tools(route: ToolsRoute) -> Html {
    match route {
        ToolsRoute::Load => html! { <LoaderPage /> },
        ToolsRoute::CreateRecipe => html! { <p>{"Create recipe"}</p> },
        ToolsRoute::DuplicateFinder => html! { <p>{"Duplicate finder"}</p> },
    }
}

fn switch_recipe(route: RecipeRoute) -> Html {
    match route {
        RecipeRoute::RecipeRoot => html! { <DefaultPage /> },
        RecipeRoute::ByCuisine => html! { <DefaultPage /> },
        RecipeRoute::FromIngredients => html! { <DefaultPage /> }
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <DefaultPage /> },
        Route::ToolsRoot | Route::Tools => {
            html! { <Switch<ToolsRoute> render={switch_tools} /> }
        },
        Route::RecipeRoot | Route::Recipe => {
            html! { <Switch<RecipeRoute> render={switch_recipe} /> }
        },
        Route::Timers => html! { <TimerPage /> },
        Route::Converters => html! { <p>{"Converters"}</p> },
        Route::NotFound => {
            html! { <div class="position-absolute top-50 start-50 translate-middle"><h1>{"Not found"}</h1></div> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class={"content"}>
            <BrowserRouter>
                <HeaderComponent />
                <SidebarComponent />
                <Switch<Route> render={switch} />
                <FooterComponent />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
