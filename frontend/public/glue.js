const invoke = window.__TAURI__.invoke;

export async function recipe_load_path(path) {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  if (selected === null) {
    // user cancelled the selection
    console.log("No directory selected!");
  } else {
    // user selected a single directory
    console.log(selected);
    return await invoke("load_path", { dataPath: selected });
  }
}
