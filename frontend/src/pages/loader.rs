use crate::recipe_load_path;
use log::{info, warn};
use yew::{function_component, html, Callback, Html};

#[function_component]
pub fn LoaderPage() -> Html {
    let on_folder_select = Callback::from(move |_| {
        info!("Loading from a folder");
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(err) = recipe_load_path().await.err() {
                warn!("{:?}", err);
            }
        })
    });

    let on_file_select = Callback::from(move |_| {
        info!("Loading from a file");
    });

    let on_url_entered = Callback::from(move |_| {
        info!("Loading from a url");
    });

    html! {
        <div class="position-absolute top-50 start-50 translate-middle">
            <h3>{"Loader"}</h3>
            <p>{"Load from a folder: "}<button onclick={on_folder_select}>{"select folder"}</button></p>
            <p>{"Load from a json file: "}<button onclick={on_file_select}>{"select file"}</button></p>
            <p>{"Load from a url: "}<input placeholder="Enter url here" /><button onclick={on_url_entered}>{"load"}</button></p>
        </div>
    }
}
