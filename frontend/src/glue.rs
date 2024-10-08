use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = get_recipe_meta_information, catch)]
    pub async fn get_recipe_meta() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = filter_ingredients, catch)]
    pub async fn filter_ingredients(
        pattern: String,
        limit: i32,
        offset: i32,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = filter_recipes, catch)]
    pub async fn filter_recipes(
        pattern: String,
        limit: i32,
        offset: i32,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = load_recipe, catch)]
    pub async fn load_recipe(
        recipe_id: i32,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = reset_database, catch)]
    pub async fn reset_database() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = get_job_status, catch)]
    pub async fn get_job_status(job_id: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = get_recipe_image, catch)]
    pub async fn get_recipe_image(recipe_id: i32) -> Result<JsValue, JsValue>;
}
