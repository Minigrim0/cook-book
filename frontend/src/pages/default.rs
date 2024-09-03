use yew::{function_component, html, Html};

#[function_component]
pub fn DefaultPage() -> Html {
    html! {
        <div class="position-absolute top-50 start-50 translate-middle text-muted text-center">
            {"Welcome to cook book !"}<br/>
            {"There is nothing loaded yet, start by "}<a href="/load">{"loading"}</a>{" or "}<a href="#" class="link link-disabled">{"creating"}</a>{" new recipe !"}
        </div>
    }
}
