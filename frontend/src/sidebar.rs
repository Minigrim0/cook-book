use yew::{function_component, html, Html};

use super::recipes_tree::RecipesTreeComponent;

#[function_component]
pub fn SideBarComponent() -> Html {
    html! {
        <div class="offcanvas offcanvas-start" tabindex="-1" id="offcanvasExample" aria-labelledby="offcanvasExampleLabel">
            <div class={"offcanvas-header"}>
                <h5 class={"offcanvas-title"} id={"offcanvasLabel"}>{"Cook Book"}</h5>
                <button type="button" class="btn-close text-reset" data-bs-dismiss="offcanvas" aria-label="Close"></button>
            </div>
            <div class={"offcanvas-body"}>
                <h2>{"Recipes"}</h2>
                <RecipesTreeComponent />
                <h3>{"Ingredients"}</h3>
                <h3>{"Categories"}</h3>
            </div>
        </div>
    }
}
