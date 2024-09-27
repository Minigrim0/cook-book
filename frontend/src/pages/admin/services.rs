use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::bindings::get_paginated_ingredients as glued_get_paginated_ingredients;
use crate::models::Ingredient;

pub fn get_paginated_ingredients(page: u32, items_per_page: u32) -> Callback<(), ()> {
    Callback::from(move |_| {
        let page = page;
        let items_per_page = items_per_page;
        spawn_local(async move {
            match glued_get_paginated_ingredients(page, items_per_page).await {
                Ok(ingredients) => {
                    // Handle successful response
                    // You might want to update some state or dispatch an action here
                    log::info!("Fetched {} ingredients", ingredients.len());
                }
                Err(e) => {
                    // Handle error
                    log::error!("Failed to fetch ingredients: {}", e);
                }
            }
        });
    })
}

