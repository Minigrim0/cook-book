use yew::prelude::*;
use crate::models::Ingredient;
use crate::components::Pagination;
use super::service;

const ITEMS_PER_PAGE: usize = 10;

pub struct IngredientList {
    ingredients: Vec<Ingredient>,
    current_page: usize,
    total_ingredients: usize,
}

pub enum Msg {
    LoadIngredients(usize),
    IngredientsLoaded(Vec<Ingredient>, usize),
    UpdateIngredient(usize, String),
}

impl Component for IngredientList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadIngredients(1));
        Self {
            ingredients: Vec::new(),
            current_page: 1,
            total_ingredients: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadIngredients(page) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match service::get_paginated_ingredient(page, ITEMS_PER_PAGE).await {
                        Ok(paginated_ingredients) => {
                            link.send_message(Msg::IngredientsLoaded(
                                paginated_ingredients.items,
                                paginated_ingredients.total,
                            ));
                        }
                        Err(e) => {
                            log::error!("Failed to load ingredients: {:?}", e);
                        }
                    }
                });
                false
            }
            Msg::IngredientsLoaded(ingredients, total) => {
                self.ingredients = ingredients;
                self.total_ingredients = total;
                true
            }
            Msg::UpdateIngredient(index, new_name) => {
                if let Some(ingredient) = self.ingredients.get_mut(index) {
                    ingredient.name = new_name;
                    // Here you would typically call a service function to update the ingredient in the backend
                    // For now, we'll just update it locally
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="ingredient-list">
                <h2>{"Ingredients"}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for self.ingredients.iter().enumerate().map(|(index, ingredient)| {
                            let index = index;
                            html! {
                                <tr key={ingredient.id}>
                                    <td>
                                        <input
                                            type="text"
                                            value={ingredient.name.clone()}
                                            onchange={ctx.link().callback(move |e: Event| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateIngredient(index, input.value())
                                            })}
                                        />
                                    </td>
                                    <td>
                                        // Add edit and delete buttons here if needed
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
                <Pagination
                    current_page={self.current_page}
                    total_items={self.total_ingredients}
                    items_per_page={ITEMS_PER_PAGE}
                    on_page_change={ctx.link().callback(Msg::LoadIngredients)}
                />
            </div>
        }
    }
}
