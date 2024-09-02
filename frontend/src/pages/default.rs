use yew::{function_component, html, Html};

#[function_component]
pub fn DefaultEmptyPage() -> Html {
    html! {
        <div class="position-absolute top-50 start-50 translate-middle text-muted text-center">
            {"Welcome to cook book !"}<br/>
            {"There is nothing loaded yet, start by loading or creating new recipe !"}
        </div>
    }
}
