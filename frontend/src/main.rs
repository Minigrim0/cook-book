use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

mod footer;
use footer::FooterComponent;

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
        <div>
            <h2 class={"heading"}>{"Hello, World!"}</h2>
            <FooterComponent />
        </div>
    }
}
