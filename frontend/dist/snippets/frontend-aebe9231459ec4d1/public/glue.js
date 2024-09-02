const invoke = window.__TAURI__.invoke;
import { open } from "@tauri-apps/api/dialog";

export async function recipe_load_path() {
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
