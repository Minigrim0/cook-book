const invoke = window.__TAURI__.invoke;

export async function recipe_load_path() {
  return await invoke("load_path");
}

export async function get_recipe_meta_information() {
  return await invoke("recipe_meta");
}

export async function get_ingredients(limit, offset) {
  return await invoke("get_ingredients", { limit: limit, offset: offset });
}

export async function filter_ingredients(pattern, limit, offset) {
    return await invoke("filter_ingredients", { pattern: pattern, limit: limit, offset: offset });
}
