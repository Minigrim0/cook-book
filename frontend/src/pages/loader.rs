use crate::recipe_load_path;
use log::{info, warn};
use yew::{function_component, html, Callback, Html};

#[function_component]
pub fn LoaderPage() -> Html {
    let onclick = Callback::from(move |_| {
        info!("Clicked !");
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(err) = recipe_load_path().await.err() {
                warn!("{:?}", err);
            }
        })
    });

    html! {
        <div class="position-absolute top-50 start-50 translate-middle">
            <h3>{"Loader"}</h3>
            <button class="btn btn-primary" {onclick}>{"load folder"}</button>
        </div>
    }
}
