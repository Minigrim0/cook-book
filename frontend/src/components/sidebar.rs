use yew::{function_component, classes, html, Html};
use yew_router::prelude::Link;

use crate::routes::{RecipeRoute, ToolsRoute};

#[function_component]
pub fn SidebarComponent() -> Html {
    html! {
        <div class="offcanvas offcanvas-start" tabindex="-1" id="offcanvasExample" aria-labelledby="offcanvasExampleLabel">
            <div class={"offcanvas-header"}>
                <h5 class={"offcanvas-title"} id={"offcanvasLabel"}>{"Cook Book"}</h5>
                <button type="button" class="btn-close text-reset" data-bs-dismiss="offcanvas" aria-label="Close"></button>
            </div>
            <div class={"offcanvas-body"}>
                <div class="accordion" id="SideBarMenuAccordion">
                    <div class="accordion-item">
                        <h2 class="accordion-header" id="AccordionRecipeHeading">
                            <button class="accordion-button" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionRecipe" aria-expanded="true" aria-controls="AccordionRecipe">
                                {"Recipes"}
                                <span class="badge bg-primary rounded-pill">{"14"}</span>
                            </button>
                        </h2>
                        <div id="AccordionRecipe" class="accordion-collapse collapse show" aria-labelledby="AccordionRecipeHeading" data-bs-parent="#SideBarMenuAccorion">
                            <div class="accordion-body">
                                <div class="list-group list-group-flush">
                                    <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::RecipeRoot}>{"all recipes"}</Link<RecipeRoute>>
                                    <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::ByCuisine}>{"by cuisine"}</Link<RecipeRoute>>
                                    <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::FromIngredients}>{"from ingredients"}</Link<RecipeRoute>>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="accordion-item">
                        <h2 class="accordion-header" id="AccordionToolsHeading">
                            <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionTools" aria-expanded="false" aria-controls="AccordionTools">
                                {"Tools"}
                            </button>
                        </h2>
                        <div id="AccordionTools" class="accordion-collapse collapse" aria-labelledby="AccordionToolsHeading" data-bs-parent="#SideBarMenuAccorion">
                            <div class="accordion-body">
                                <div class="list-group list-group-flush">
                                    <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::Load}>{"Load recipes"}</Link<ToolsRoute>>
                                    <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::CreateRecipe}>{"Create a recipe"}</Link<ToolsRoute>>
                                    <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::DuplicateFinder}>{"Duplicate finder"}</Link<ToolsRoute>>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="accordion-item">
                        <h2 class="accordion-header" id="AccordionCategoriesHeading">
                            <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionCategories" aria-expanded="false" aria-controls="AccordionCategories">
                                {"Settings"}
                            </button>
                        </h2>
                        <div id="AccordionCategories" class="accordion-collapse collapse" aria-labelledby="AccordionCategoriesHeading" data-bs-parent="#SideBarMenuAccorion">
                            <div class="accordion-body">
                                <strong>{"WIP"}</strong>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
