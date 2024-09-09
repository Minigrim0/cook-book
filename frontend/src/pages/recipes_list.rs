use log::{error, info};
use std::cmp::{max, min};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlButtonElement, HtmlInputElement};
use yew::{classes, html, Component, Context, Html};
use yew::{InputEvent, MouseEvent};
use yew_router::prelude::Link;

use crate::routes::RecipeRoute;

use models::models::Recipe;
use models::PaginatedRecipe;

use super::services::filter_recipes;

pub struct RecipesPage {
    recipes: Vec<Recipe>,
    pattern: String,
    total_recipes: i32,
    current_page: i32,
    num_pages: i32,
}

pub enum Msg {
    LoadRecipes,
    RecipesLoaded(Result<PaginatedRecipe, String>),
    PageUp,
    PageDown,
    GoToPage(MouseEvent),
    InputChanged(InputEvent),
}

impl Component for RecipesPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadRecipes);

        RecipesPage {
            recipes: Vec::new(),
            pattern: String::new(),
            total_recipes: 0,
            current_page: 0,
            num_pages: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadRecipes => {
                let callback = ctx.link().callback(Msg::RecipesLoaded);
                filter_recipes(self.pattern.clone(), self.current_page, callback);
                false
            }
            Msg::RecipesLoaded(result) => {
                match result {
                    Ok(ingredients) => {
                        self.recipes = ingredients.0;
                        self.total_recipes = ingredients.1 as i32;
                        self.num_pages = ingredients.2 as i32;
                    }
                    Err(e) => error!("An error occured: {}", e.to_string()),
                }
                true
            }
            Msg::InputChanged(e) => {
                let target: Option<EventTarget> = e.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    self.pattern = input.value();
                    ctx.link().send_message(Msg::LoadRecipes);
                    true
                } else {
                    false
                }
            }
            Msg::PageUp => {
                self.current_page = min(self.current_page + 1, self.num_pages - 1);
                ctx.link().send_message(Msg::LoadRecipes);
                true
            }
            Msg::PageDown => {
                self.current_page = max(self.current_page - 1, 0);
                ctx.link().send_message(Msg::LoadRecipes);
                true
            }
            Msg::GoToPage(e) => {
                let target: Option<EventTarget> = e.target();
                let button = target.and_then(|t| t.dyn_into::<HtmlButtonElement>().ok());

                if let Some(button) = button {
                    info!("Go to page: {}", button.value());
                    let page = button.value().parse::<i32>().unwrap_or(0);

                    self.current_page = page;
                    ctx.link().send_message(Msg::LoadRecipes);
                    true
                } else {
                    error!("Unable to get button value");
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_cb = ctx.link().callback(Msg::InputChanged);
        let previous_cb = ctx.link().callback(|_| Msg::PageDown);
        let next_cb = ctx.link().callback(|_| Msg::PageUp);
        let button_cb = ctx.link().callback(Msg::GoToPage);

        html! {
            <div>
                <div class={classes!("row", "p-2")}>
                    <p class={classes!("col")}>
                        <span class={classes!("h2")}>{"Recipes list"}</span>
                        <span class={classes!("small")}>{self.total_recipes}{" element(s)"}</span>
                    </p>
                    <div class={classes!("col-3")}>
                        <div class={classes!("input-group", "input-group-sm", "mb-3")}>
                            <span class={classes!("input-group-text")} id="filter-input-text">{"Filter"}</span>
                            <input
                                oninput={input_cb}
                                aria-label="Filter ingredients"
                                aria-describedby="filter-input-text"
                                class={classes!("form-control")}
                                type="text"
                                placeholder="type to filter"
                            />
                        </div>
                    </div>
                </div>
                <nav aria-label="Page navigation example">
                    <ul class="pagination">
                        <li class="page-item">
                            <button
                                class={classes!("page-link", if self.current_page > 0 {""} else {"disabled"})}
                                onclick={previous_cb}
                            >
                                {"Previous"}
                            </button>
                        </li>
                        {
                            (max(0, self.current_page-3)..self.current_page)
                                .map(|index| {
                                    html! {
                                        <li class="page-item">
                                            <button
                                                class={classes!("page-link")}
                                                onclick={button_cb.clone()}
                                                value={index.to_string()}
                                            >
                                                {index + 1}
                                            </button>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                        <li class="page-item"><button class={classes!("page-link", "active")}>{self.current_page + 1}</button></li>
                        {
                            ((self.current_page+1)..min(self.num_pages, self.current_page+4))
                                .map(|index| {
                                    html! {
                                        <li class="page-item">
                                            <button
                                                class={classes!("page-link")}
                                                onclick={button_cb.clone()}
                                                value={index.to_string()}
                                            >
                                                {index + 1}
                                            </button>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                        <li class="page-item">
                            <button
                                class={classes!("page-link", if self.current_page < self.num_pages - 1 {""} else {"disabled"})}
                                onclick={next_cb}
                            >
                                {"Next"}
                            </button>
                        </li>
                    </ul>
                </nav>
                <div class={classes!("row", "row-cols-xs-2", "row-cols-sm-3", "row-cols-md-6", "row-cols-lg-12", "g-4")}>
                    if self.total_recipes > 0 {
                        {
                            self.recipes.iter().map(|r| {
                                html! {
                                    <div class={classes!("col")}>
                                        <div class={classes!("card")}>
                                            <img src="..." class={classes!("card-img-top")} alt="an delicious image here" />
                                            <div class={classes!("card-body")}>
                                                <Link<RecipeRoute> classes={classes!("h5", "card-title", "link")} to={RecipeRoute::RecipeDetails { id: r.id }}>
                                                    {&r.name}
                                                </Link<RecipeRoute>>
                                                <p class={classes!("card-text")}>{&r.name}{"'s description"}</p>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        }
                    } else {
                        <div class={classes!("position-absolute", "top-50", "start-50", "translate-middle", "text-muted", "text-center")}>
                            {"Nothing here !"}
                        </div>
                    }
                </div>
            </div>
        }
    }
}
