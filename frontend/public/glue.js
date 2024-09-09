const invoke =
  window.__TAURI__ != undefined
    ? window.__TAURI__.invoke
    : (c, a = {}) => {
        "null";
      };

export async function recipe_load_path() {
  return await invoke("load_path");
}

export async function get_recipe_meta_information() {
  return await invoke("recipe_meta");
}

export async function filter_ingredients(pattern, limit, offset) {
  return await invoke("filter_ingredients", {
    pattern: pattern,
    limit: limit,
    offset: offset,
  });
}

export async function filter_recipes(pattern, limit, offset) {
  return await invoke("filter_recipes", {
    pattern: pattern,
    limit: limit,
    offset: offset,
  });
}

export async function load_recipe(recipe_id) {
  return await invoke("load_recipe", {
    recipe_id: recipe_id,
  });
}
