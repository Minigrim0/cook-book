use tauri_api::dialog;
use yew::{function_component, html, Html, Callback};
use crate::recipe_load_path;

#[function_component]
pub fn LoaderPage() -> Html {
    let onclick = Callback::from(move |_| {
        recipe_load_path();
    });

    html! {
        <div class="position-absolute top-50 start-50 translate-middle">
            <h3>{"Loader"}</h3>
            <button class="btn btn-primary" {onclick}>{"load folder"}</button>
        </div>
    }
}
