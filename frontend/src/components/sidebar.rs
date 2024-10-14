use log::error;
use serde_wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::{classes, html, Component, Context, Html};
use yew_router::prelude::Link;

use crate::routes::{AdminRoute, RecipeRoute, ToolsRoute};
use crate::{get_recipe_meta, reset_database};
use models::RecipeMeta;

pub struct SidebarComponent {
    recipes_metadata: RecipeMeta,
}

fn load_meta(callback: Callback<Result<RecipeMeta, String>>) {
    spawn_local(async move {
        match get_recipe_meta().await {
            Ok(response) => {
                let meta: Result<RecipeMeta, String> =
                    serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(meta);
            }
            Err(e) => {
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}

pub enum Msg {
    LoadRecipeMeta,
    RecipeMetaLoaded(Result<RecipeMeta, String>),
    ResetDatabase,
}

impl Component for SidebarComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadRecipeMeta);

        Self {
            recipes_metadata: RecipeMeta::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadRecipeMeta => {
                let meat_ready_cb = ctx.link().callback(Msg::RecipeMetaLoaded);
                load_meta(meat_ready_cb);
                false
            }
            Msg::RecipeMetaLoaded(result) => {
                match result {
                    Ok(recipe_meta) => self.recipes_metadata = recipe_meta,
                    Err(e) => error!("Error {}", e.as_str()),
                }
                true
            }
            Msg::ResetDatabase => {
                spawn_local(async move {
                    if let Err(e) = reset_database().await {
                        error!("Error {:?}", e);
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_database_cb = ctx.link().callback(|_| Msg::ResetDatabase);

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
                                    <i class="bi bi-book me-2"></i>{"Recipes"}
                                </button>
                            </h2>
                            <div id="AccordionRecipe" class="accordion-collapse collapse show" aria-labelledby="AccordionRecipeHeading" data-bs-parent="#SideBarMenuAccorion">
                                <div class="accordion-body">
                                    <div class="list-group list-group-flush">
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::RecipeRoot}>
                                            <p class="col">
                                                <i class="bi bi-list-ul me-2"></i>
                                                <span class="me-2 badge bg-primary rounded-pill">{self.recipes_metadata.recipe_amount}</span>
                                                {"all recipes"}
                                            </p>
                                        </Link<RecipeRoute>>
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::ByCuisine}>
                                            <p>
                                                <i class="bi bi-globe me-2"></i>
                                                <span class="me-2 badge bg-primary rounded-pill">{self.recipes_metadata.cuisine_amount}</span>
                                                {"by cuisine"}
                                            </p>
                                        </Link<RecipeRoute>>
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::FromIngredients}>
                                            <p class="col">
                                                <i class="bi bi-egg-fried me-2"></i>
                                                <span class="me-2 badge bg-primary rounded-pill">{self.recipes_metadata.ingredients_amount}</span>
                                                {"by ingredient"}
                                            </p>
                                        </Link<RecipeRoute>>
                                        <Link<AdminRoute> classes={classes!("list-group-item")} to={AdminRoute::AdminRoot}>
                                            <p>
                                                <i class="bi bi-gear me-2"></i>
                                                {"admin"}
                                            </p>
                                        </Link<AdminRoute>>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="accordion-item">
                            <h2 class="accordion-header" id="AccordionToolsHeading">
                                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionTools" aria-expanded="false" aria-controls="AccordionTools">
                                    <i class="bi bi-tools me-2"></i>{"Tools"}
                                </button>
                            </h2>
                            <div id="AccordionTools" class="accordion-collapse collapse" aria-labelledby="AccordionToolsHeading" data-bs-parent="#SideBarMenuAccorion">
                                <div class="accordion-body">
                                    <div class="list-group list-group-flush">
                                        <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::Load}>
                                            <i class="bi bi-cloud-upload me-2"></i>{"Load recipes"}
                                        </Link<ToolsRoute>>
                                        <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::CreateRecipe}>
                                            <i class="bi bi-plus-circle me-2"></i>{"Create a recipe"}
                                        </Link<ToolsRoute>>
                                        <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::DuplicateFinder}>
                                            <i class="bi bi-search me-2"></i>{"Ingredients to recipe"}
                                        </Link<ToolsRoute>>
                                        <Link<ToolsRoute> classes={classes!("list-group-item", "list-group-item-action")} to={ToolsRoute::DuplicateFinder}>
                                            <i class="bi bi-files me-2"></i>{"Duplicate finder"}
                                        </Link<ToolsRoute>>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="accordion-item">
                            <h2 class="accordion-header" id="AccordionCategoriesHeading">
                                <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#AccordionCategories" aria-expanded="false" aria-controls="AccordionCategories">
                                    <i class="bi bi-gear me-2"></i>{"Settings"}
                                </button>
                            </h2>
                            <div id="AccordionCategories" class={classes!("accordion-collapse", "collapse")} aria-labelledby="AccordionCategoriesHeading" data-bs-parent="#SideBarMenuAccorion">
                                <div class="accordion-body">
                                    <div class={classes!("list-group", "list-group-flush")}>
                                        <button class={classes!("link", "list-group-item", "list-group-item-action")}>
                                            <i class="bi bi-sliders me-2"></i>{"application settings"}
                                        </button>
                                        <button class={classes!("link", "list-group-item", "list-group-item-action")}>
                                            <i class="bi bi-cloud-arrow-up me-2"></i>{"backup database"}
                                        </button>
                                        <button class={classes!("link", "list-group-item", "list-group-item-action", "link-danger")}>
                                            <i class="bi bi-power me-2"></i>{"quit"}
                                        </button>
                                        <button class={classes!("link", "list-group-item", "list-group-item-action", "link-danger")} onclick={reset_database_cb}>
                                            <i class="bi bi-arrow-counterclockwise me-2"></i>{"reset database"}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
