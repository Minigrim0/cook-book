use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

mod footer;
mod sidebar;
mod recipes_tree;
mod central;
mod header;
use header::HeaderComponent;
use footer::FooterComponent;
use sidebar::SideBarComponent;
use central::DefaultCentralPart;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path(path: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class={"content"}>
            <HeaderComponent />
            <SideBarComponent />
            <DefaultCentralPart />
            <FooterComponent />
        </div>
    }
}
