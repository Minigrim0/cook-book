use crate::recipe_load_path;
use log::{info, warn};
use yew::{classes, function_component, html, Callback, Html};


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
        <div class={classes!("position-absolute", "top-50", "start-50", "translate-middle", "w-75")}>
            <h3 class={classes!("w-100", "text-center")}>{"Loader"}</h3>
            <div class={classes!("w-100", "text-center")}>
                <small class={classes!("text-muted")}>{"Use this tool to load recipes in"}</small>
            </div>
            <div class={classes!("row", "col-12", "p-2")}>
                <p class={classes!("col-10")}>{"Load from a folder: "}</p>
                <button class={classes!("col")} onclick={on_folder_select}>{"select folder"}</button>
            </div>
            <div class={classes!("row", "col-12", "p-2")}>
                <p class={classes!("col-10")}>{"Load from a json file: "}</p>
                <button class={classes!("col")} onclick={on_file_select}>{"select file"}</button>
            </div>
            <div class={classes!("row", "col-12", "p-2")}>
                <p class={classes!("col-5")}>{"Load from a url: "}</p>
                <input  class={classes!("col-5")} placeholder="Enter url here" />
                <button class={classes!("col")} onclick={on_url_entered}>{"load"}</button>
            </div>
        </div>
    }
}
