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

export async function load_recipe(id) {
  console.log("Loading recipe from js glue");
  return await invoke("load_recipe", {
    recipeId: id,
  });
}

export async function reset_database() {
  return await invoke("reset_database");
}

export async function get_job_status(job_id) {
  return await invoke("get_job_status", {
    jobId: job_id,
  });
}

export async function get_recipe_image(id) {
  return await invoke("get_recipe_image", {
    recipeId: id,
  });
}
