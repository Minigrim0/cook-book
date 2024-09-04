const invoke = window.__TAURI__.invoke;

export async function recipe_load_path() {
  return await invoke("load_path");
}

export async function get_recipe_met_information() {
  return await invoke("recipe_meta");
}
