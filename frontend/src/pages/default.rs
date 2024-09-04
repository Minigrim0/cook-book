use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::ToolsRoute;

#[function_component]
pub fn DefaultPage() -> Html {
    html! {
        <div class="position-absolute top-50 start-50 translate-middle text-muted text-center">
            {"Welcome to cook book !"}<br/>
            {"There is nothing loaded yet, start by "}
            <Link<ToolsRoute> to={ToolsRoute::Load}>{"loading recipes"}</Link<ToolsRoute>>
            {" or "}
            <Link<ToolsRoute> to={ToolsRoute::CreateRecipe}>{"creating new recipe"}</Link<ToolsRoute>>
            {" !"}
        </div>
    }
}
