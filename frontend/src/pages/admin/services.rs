use crate::glue::filter_ingredients;
// use models::Ingredient;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn get_paginated_ingredients(page: u32, items_per_page: u32) -> Callback<(), ()> {
    Callback::from(move |_| {
        let page = page;
        let items_per_page = items_per_page;
        spawn_local(async move {
            match filter_ingredients(String::new(), page as i32, items_per_page as i32).await {
                Ok(ingredients) => {
                    // Handle successful response
                    // You might want to update some state or dispatch an action here
                    log::info!("Fetched {:?} ingredients", ingredients);
                }
                Err(e) => {
                    // Handle error
                    log::error!("Failed to fetch ingredients: {:?}", e);
                }
            }
        });
    })
}
