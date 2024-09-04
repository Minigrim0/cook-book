use log::error;
use serde_wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew::{classes, html, Component, Context, Html};
use yew_router::prelude::Link;

use crate::get_recipe_meta;
use crate::routes::{RecipeRoute, ToolsRoute};
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                                </button>
                            </h2>
                            <div id="AccordionRecipe" class="accordion-collapse collapse show" aria-labelledby="AccordionRecipeHeading" data-bs-parent="#SideBarMenuAccorion">
                                <div class="accordion-body">
                                    <div class="list-group list-group-flush">
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::RecipeRoot}>
                                            <span class={classes!("row")}>
                                                <p class="col-10">{"all recipes"}</p>
                                                <span class="col">
                                                    <span class="badge bg-primary rounded-pill">{self.recipes_metadata.recipe_amount}</span>
                                                </span>
                                            </span>
                                        </Link<RecipeRoute>>
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::ByCuisine}>
                                            <span class={classes!("row")}>
                                                <p class="col-10">{"by cuisine"}</p>
                                                <span class="col">
                                                    <span class="badge bg-primary rounded-pill">{self.recipes_metadata.cuisine_amount}</span>
                                                </span>
                                            </span>
                                        </Link<RecipeRoute>>
                                        <Link<RecipeRoute> classes={classes!("list-group-item")} to={RecipeRoute::FromIngredients}>
                                            <span class={classes!("row")}>
                                                <p class="col-10">{"from ingredients"}</p>
                                                <span class="col">
                                                    <span class="badge bg-primary rounded-pill">{self.recipes_metadata.ingredients_amount}</span>
                                                </span>
                                            </span>
                                        </Link<RecipeRoute>>
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
}
