use yew::{function_component, html, Html};

#[function_component]
pub fn TimerPage() -> Html {
    html! {
        <div class="position-absolute top-50 start-50 translate-middle">
            <h1>{"Add a timer"}</h1>
        </div>
    }
}
