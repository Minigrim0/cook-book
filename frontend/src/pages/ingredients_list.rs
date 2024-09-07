use log::error;
use serde_wasm_bindgen;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{classes, html, Component, Context, Html};
use yew::{Callback, InputEvent};
use yew_agent::oneshot::use_oneshot_runner;
use yew_agent::prelude::*;

use crate::get_ingredient_list;
use models::models::Ingredient;

fn load_ingredients(callback: Callback<Result<Vec<Ingredient>, String>>) {
    spawn_local(async move {
        match get_ingredient_list().await {
            Ok(response) => {
                let data: Result<Vec<Ingredient>, String> =
                    serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(data);
            }
            Err(e) => {
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}

#[oneshot]
async fn FilterTask(info: (Rc<Vec<Ingredient>>, String)) -> Vec<Ingredient> {
    fn filter(list: Rc<Vec<Ingredient>>, pattern: String) -> Vec<Ingredient> {
        list.iter()
            .filter(|e| e.name.contains(pattern.as_str()))
            .map(|i| i.clone())
            .collect::<Vec<Ingredient>>()
    }

    filter(info.0, info.1)
}

/// Filters the vector of ingredients in a separate thread to avoid blocking the frontend on input change
fn filter_ingredients(
    ingredients: Rc<Vec<Ingredient>>,
    filter_pattern: String,
    callback: Callback<Vec<Ingredient>>,
) {
    let filter_task = use_oneshot_runner::<FilterTask>();

    spawn_local(async move {
        let filter_agent = filter_task.clone();
        let filtered = filter_agent.run((ingredients, filter_pattern)).await;

        callback.emit(filtered);
    });
}

pub struct IngredientsPage {
    ingredients: Rc<Vec<Ingredient>>,
    filtered: Vec<Ingredient>,
}

pub enum Msg {
    LoadIngredients,
    IngredientsLoaded(Result<Vec<Ingredient>, String>),
    InputChanged(InputEvent),
    FilterResult(Vec<Ingredient>),
}

impl Component for IngredientsPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadIngredients);

        IngredientsPage {
            ingredients: Rc::new(Vec::new()),
            filtered: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadIngredients => {
                let callback = ctx.link().callback(Msg::IngredientsLoaded);
                load_ingredients(callback);
                false
            }
            Msg::IngredientsLoaded(result) => {
                match result {
                    Ok(ingredients) => self.ingredients = Rc::new(ingredients),
                    Err(e) => error!("An error occured: {}", e.to_string()),
                }
                true
            }
            Msg::InputChanged(e) => {
                let target: Option<EventTarget> = e.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                if let Some(input) = input {
                    let callback = ctx.link().callback(Msg::FilterResult);
                    filter_ingredients(Rc::clone(&self.ingredients), input.value(), callback);
                }
                true
            }
            Msg::FilterResult(data) => {
                self.filtered = data;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_cb = ctx.link().callback(Msg::InputChanged);
        let num_filtered_elements = self.filtered.len();

        html! {
            <div class="container">
                <div class={classes!("row", "d-flex", "justify-content-between")}>
                    <h1 class={classes!("col")}>{"Ingredients list"}</h1>
                    <small class={classes!("col")}>{num_filtered_elements}{" element(s)"}</small>
                    <input oninput={input_cb} class={classes!("col")} type="text" placeholder="type to filter" />
                </div>
                <div class={classes!("row", "row-cols-4", "row-cols-md-8", "g-4")}>
                    if num_filtered_elements > 0 {
                        {
                            self.filtered.iter().map(|i| {
                                html! {
                                    <div class={classes!("col")}>
                                        <div class={classes!("card")}>
                                            <img src="..." class={classes!("card-img-top")} alt="an delicious image here" />
                                            <div class={classes!("card-body")}>
                                                <h5 class={classes!("card-title")}>{&i.name}</h5>
                                                <p class={classes!("card-text")}>{&i.name}{"'s description"}</p>
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
