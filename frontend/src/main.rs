use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

mod central;
mod footer;
mod header;
mod recipes_tree;
mod routes;
mod sidebar;

use central::DefaultCentralPart;
use footer::FooterComponent;
use header::HeaderComponent;
use routes::Route;
use sidebar::SideBarComponent;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path(path: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <DefaultCentralPart /> },
        Route::Timers => html! { <p>{"Timers"}</p> },
        Route::Converters => html! { <p>{"Converters"}</p> },
        _ => html! { <p>{"Not found"}</p> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class={"content"}>
            <HeaderComponent />
            <SideBarComponent />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>

            <FooterComponent />
        </div>
    }
}
