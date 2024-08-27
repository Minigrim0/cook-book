use yew::{function_component, html, Html};

#[function_component]
pub fn FooterComponent() -> Html {
    html! {
        <small>{ "This is a footer line" }</small>
    }
}
