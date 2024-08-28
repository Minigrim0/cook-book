use yew::{function_component, html, Html};

use super::recipes_tree::RecipesTreeComponent;

#[function_component]
pub fn SideBarComponent() -> Html {
    html! {
        <div class={"sidebar"}>
            <h2 class={"mt-0"}>{"Cook Book"}</h2>
            <RecipesTreeComponent />
        </div>
    }
}
