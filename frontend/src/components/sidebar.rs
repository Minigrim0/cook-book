use yew::{function_component, html, Html};

use super::RecipeTreeComponent;

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
                                <RecipeTreeComponent />
                            </div>
                        </div>
                    </div>
                    <div class="accordion-item">
                        <h2 class="accordion-header" id="AccordionIngredientsHeading">
                            <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionIngredients" aria-expanded="false" aria-controls="AccordionIngredients">
                                {"Ingredients"}
                            </button>
                        </h2>
                        <div id="AccordionIngredients" class="accordion-collapse collapse" aria-labelledby="AccordionIngredientsHeading" data-bs-parent="#SideBarMenuAccorion">
                            <div class="accordion-body">
                                <strong>{"WIP"}</strong>
                            </div>
                        </div>
                    </div>
                    <div class="accordion-item">
                        <h2 class="accordion-header" id="AccordionCategoriesHeading">
                            <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionCategories" aria-expanded="false" aria-controls="AccordionCategories">
                                {"Categories"}
                            </button>
                        </h2>
                        <div id="AccordionCategories" class="accordion-collapse collapse" aria-labelledby="AccordionCategoriesHeading" data-bs-parent="#SideBarMenuAccorion">
                            <div class="accordion-body">
                                <strong>{"WIP"}</strong>
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
                                <div class="list-group">
                                    <a class="list-group-item list-group-item-action" href="/load">{"Load recipes"}</a>
                                    <a class="list-group-item list-group-item-action" href="#">{"Duplicate finder"}</a>
                                    <a class="list-group-item list-group-item-action" href="#">{"WIP"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
