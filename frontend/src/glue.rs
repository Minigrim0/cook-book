use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = recipe_load_path, catch)]
    pub async fn recipe_load_path() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = get_recipe_meta_information, catch)]
    pub async fn get_recipe_meta() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = get_ingredients, catch)]
    pub async fn get_ingredient_list() -> Result<JsValue, JsValue>;
}
