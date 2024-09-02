use yew::{function_component, html, Html};

#[function_component]
pub fn LoaderPage() -> Html {
    html! {
        <div class="position-absolute top-50 start-50 translate-middle">
            <h3>{"Loader"}</h3>
            <button class="btn btn-primary">{"load folder"}</button>
        </div>
    }
}
