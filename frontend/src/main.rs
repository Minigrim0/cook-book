use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;

use components::{FooterComponent, HeaderComponent, SidebarComponent};
use pages::{DefaultEmptyPage, LoaderPage};
use routes::Route;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path() -> Result<JsValue, JsValue>;
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <DefaultEmptyPage /> },
        Route::Loaders => html! { <LoaderPage /> },
        Route::Timers => html! { <p>{"Timers"}</p> },
        Route::Converters => html! { <p>{"Converters"}</p> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class={"content"}>
            <HeaderComponent />
            <SidebarComponent />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>

            <FooterComponent />
        </div>
    }
}
