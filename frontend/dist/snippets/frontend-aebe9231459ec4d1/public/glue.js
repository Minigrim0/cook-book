const invoke = window.__TAURI__.invoke;

export async function recipe_load_path(path) {
  return await invoke("load_path", { path: path });
}
