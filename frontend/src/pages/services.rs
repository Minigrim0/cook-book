use yew::Callback;
use models::models::PaginatedIngredients;
use wasm_bindgen_futures::spawn_local;

use crate::glue;

/// Spawns a future on the yew runtime to filter the list of ingredients from the backend
pub fn filter_ingredients(pattern: String, page: i32, callback: Callback<Result<PaginatedIngredients, String>>) {
    spawn_local(async move {
        match glue::filter_ingredients(pattern, 24, page * 24).await {
            Ok(response) => {
                let data: Result<PaginatedIngredients, String> =
                    serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string());
                callback.emit(data);
            }
            Err(e) => {
                callback.emit(Err(serde_wasm_bindgen::from_value(e).unwrap()));
            }
        }
    });
}
