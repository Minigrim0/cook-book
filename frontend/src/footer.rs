use yew::{function_component, html, Html};

#[function_component]
pub fn FooterComponent() -> Html {
    html! {
        <div class={"footer"}>
            <p>{"This is a footer"}</p>
        </div>
    }
}
